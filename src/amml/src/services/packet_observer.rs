#![allow(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};

use crossbeam_channel::{Sender, TrySendError};
use heads_core::services::FaceExtractor;
use heads_core::structs::{FaceUpdate, HeadUv};

use super::{ChatRuntime, GeometryReader, MemoryReader, SkinDiagnostics, SkinReader, StdStringReader};
use crate::structs::{PacketOffsets, PlayerListOffsets, SkinEvent};

/// Turns player list and player skin packets into face updates on a bounded channel.
pub struct PacketObserver {
    sender: Sender<FaceUpdate>,
    offsets: PacketOffsets,
    dropped: AtomicU64,
    diagnostics: SkinDiagnostics,
    geometry: Option<GeometryReader>,
}

impl PacketObserver {
    // most entries one list packet is allowed to carry; guards against a garbage vector
    const MAX_ENTRIES: usize = 4096;
    // one log line per this many dropped updates
    const LOG_EVERY: u64 = 256;
    // longest name or XUID the chat feed reads; a longer one is skipped
    const MAX_IDENTITY: usize = 64;

    pub fn new(sender: Sender<FaceUpdate>, offsets: PacketOffsets) -> Self {
        Self::new_with_diagnostics(sender, offsets, SkinDiagnostics::new())
    }

    pub fn new_with_diagnostics(
        sender: Sender<FaceUpdate>,
        offsets: PacketOffsets,
        diagnostics: SkinDiagnostics,
    ) -> Self {
        Self { sender, offsets, dropped: AtomicU64::new(0), diagnostics, geometry: None }
    }

    /// As `new_with_diagnostics`, and a skin with a geometry gets its face cut where the geometry puts the head.
    pub fn new_with_geometry(
        sender: Sender<FaceUpdate>,
        offsets: PacketOffsets,
        diagnostics: SkinDiagnostics,
        geometry: GeometryReader,
    ) -> Self {
        Self { geometry: Some(geometry), ..Self::new_with_diagnostics(sender, offsets, diagnostics) }
    }

    pub fn geometry(&self) -> Option<&GeometryReader> {
        self.geometry.as_ref()
    }

    pub fn diagnostics(&self) -> &SkinDiagnostics {
        &self.diagnostics
    }

    /// How many updates were dropped because the channel was full.
    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }

    /// Queues a `Clear`. False when the channel is full and the update was not queued.
    pub fn send_clear(&self) -> bool {
        self.sender.try_send(FaceUpdate::Clear).is_ok()
    }

    /// Sends an update for each add and remove entry in a `PlayerListPacket`.
    ///
    /// # Safety
    /// `packet` must be null or the address of a live `PlayerListPacket` with the layout in `offsets`, and must
    /// stay alive for the call. The detour that holds the packet guarantees this.
    pub unsafe fn on_player_list(&self, packet: usize) {
        if packet == 0 {
            return;
        }
        // SAFETY: the caller guarantees the packet is live, and the entries vector is inside it.
        let Some([first, last]) = (unsafe { MemoryReader::read::<[usize; 2]>(packet + self.offsets.list.entries) })
        else {
            return;
        };
        let entry_size = self.offsets.list.entry_size;
        if first == 0 || last < first || entry_size == 0 {
            return;
        }
        let count = ((last - first) / entry_size).min(Self::MAX_ENTRIES);
        for index in 0..count {
            let entry = first + index * entry_size;
            // SAFETY: `entry` is inside the entries vector of the live packet.
            unsafe { self.on_entry(entry) };
        }
    }

    /// Sends a skin update for a `PlayerSkinPacket`.
    ///
    /// # Safety
    /// `packet` must be null or the address of a live `PlayerSkinPacket` with the layout in `offsets`, and must
    /// stay alive for the call.
    pub unsafe fn on_player_skin(&self, packet: usize) {
        if packet == 0 {
            return;
        }
        // SAFETY: the caller guarantees the packet is live; the uuid is inside it.
        let Some(uuid) = (unsafe { MemoryReader::read::<[u8; 16]>(packet + self.offsets.skin_packet.uuid) }) else {
            return;
        };
        if self.drop_when_full() {
            return;
        }
        // SAFETY: the skin reference is inside the live packet.
        let Some(face) = (unsafe { self.face("PlayerSkin", uuid, None, packet + self.offsets.skin_packet.skin) })
        else {
            return;
        };
        ChatRuntime::with_state_blocking(|state| state.on_skin(uuid, &face));
        self.send(FaceUpdate::Skin { uuid, face });
    }

    // SAFETY contract: `entry` is the address of one live entry of a PlayerListPacket.
    unsafe fn on_entry(&self, entry: usize) {
        // SAFETY: the entry is live per the contract.
        let Some(variant) = (unsafe { MemoryReader::read::<u8>(entry + self.offsets.list.variant_index) }) else {
            return;
        };
        // SAFETY: the entry is live per the contract.
        let Some(uuid) = (unsafe { MemoryReader::read::<[u8; 16]>(entry + self.offsets.list.uuid) }) else {
            return;
        };
        match variant {
            PlayerListOffsets::VARIANT_ADD => {
                // SAFETY: the entry is live per the contract.
                let identity = unsafe { self.identity(entry) };
                if let Some((name, xuid)) = &identity {
                    ChatRuntime::with_state_blocking(|state| state.on_identity(uuid, xuid, name));
                }
                if self.drop_when_full() {
                    return;
                }
                // SAFETY: the entry is live per the contract.
                let Some(actor) = (unsafe { MemoryReader::read::<i64>(entry + self.offsets.list.actor) }) else {
                    return;
                };
                // SAFETY: the skin reference is inside the live entry.
                let Some(face) =
                    (unsafe { self.face("PlayerList.Add", uuid, Some(actor), entry + self.offsets.list.skin) })
                else {
                    return;
                };
                if let Some((name, xuid)) = &identity {
                    ChatRuntime::with_state_blocking(|state| state.on_face(uuid, xuid, name, &face));
                }
                self.send(FaceUpdate::Add { uuid, actor, face });
            }
            PlayerListOffsets::VARIANT_REMOVE => self.send(FaceUpdate::Remove { uuid }),
            _ => {}
        }
    }

    // SAFETY contract: `skin_ref` is the address of a live SerializedSkinRef.
    unsafe fn face(
        &self,
        kind: &'static str,
        uuid: [u8; 16],
        actor: Option<i64>,
        skin_ref: usize,
    ) -> Option<Box<heads_core::structs::FaceImage>> {
        // SAFETY: the skin reference is live per the contract, and the views are dropped before returning.
        let (view, animated) =
            unsafe { (SkinReader::view(skin_ref, &self.offsets), SkinReader::animated_face(skin_ref, &self.offsets)) };
        // SAFETY: the skin reference is live per the contract; the geometry is read and freed inside the call.
        let geometry = match (view.is_some() || animated.is_some(), &self.geometry) {
            (true, Some(reader)) => unsafe { reader.read(skin_ref, &self.offsets, animated.is_some()) },
            _ => None,
        };
        // a persona face is drawn from the animated face image; the fixed rectangles are used only on a persona skin
        // whose geometry has no readable animated face mesh
        let animated_head = geometry.as_ref().and_then(|read| read.animated_head).or_else(|| {
            // SAFETY: the skin reference is live per the contract.
            (animated.is_some() && unsafe { SkinReader::is_persona(skin_ref, &self.offsets) })
                .then(HeadUv::animated_face)
        });
        let from_animated = animated
            .as_ref()
            .zip(animated_head.as_ref())
            .map(|(view, head)| FaceExtractor::extract_at(view, head))
            .filter(Result::is_ok);
        let located = geometry.as_ref().and_then(|read| read.head);
        let (head_source, extracted, used_head) = match from_animated {
            Some(face) => ("animated_face", Some(face), animated_head),
            None => (
                if located.is_some() { "geometry" } else { "classic" },
                view.as_ref().map(|view| match &located {
                    Some(head) => FaceExtractor::extract_at(view, head),
                    None => FaceExtractor::extract(view),
                }),
                located,
            ),
        };
        let outcome = match &extracted {
            Some(Ok(_)) => "Ok".to_string(),
            Some(Err(error)) => format!("{error:?}"),
            None => "SkinNotReadable".to_string(),
        };
        let event = SkinEvent {
            kind,
            uuid,
            actor,
            skin_ref,
            head_source,
            head: used_head.unwrap_or_else(HeadUv::classic),
            geometry_json: geometry.as_ref().and_then(|read| read.json.as_deref()),
            outcome,
        };
        // SAFETY: the skin reference is live per the contract.
        unsafe { self.diagnostics.record(&event, &self.offsets) };
        extracted?.ok().map(Box::new)
    }

    // The name and XUID of an Add entry, or `None` without the chat offsets or when one is unreadable.
    // SAFETY contract: `entry` is the address of one live Add entry of a PlayerListPacket.
    unsafe fn identity(&self, entry: usize) -> Option<(String, String)> {
        let (Some(name_offset), Some(xuid_offset)) = (self.offsets.add_entry_name, self.offsets.add_entry_xuid) else {
            return None;
        };
        // SAFETY: the name and XUID strings are inside the live entry per the contract.
        unsafe {
            Some((
                StdStringReader::text(entry + name_offset, Self::MAX_IDENTITY)?,
                StdStringReader::text(entry + xuid_offset, Self::MAX_IDENTITY)?,
            ))
        }
    }

    // A full channel drops the update, so the skin is not extracted at all.
    fn drop_when_full(&self) -> bool {
        if self.sender.is_full() {
            self.note_drop();
            return true;
        }
        false
    }

    fn note_drop(&self) {
        let dropped = self.dropped.fetch_add(1, Ordering::Relaxed) + 1;
        if dropped % Self::LOG_EVERY == 1 {
            curia::warn!("face updates were dropped because the channel is full", { dropped: dropped });
        }
    }

    fn send(&self, update: FaceUpdate) {
        match self.sender.try_send(update) {
            Ok(()) => {}
            Err(TrySendError::Full(_) | TrySendError::Disconnected(_)) => self.note_drop(),
        }
    }
}
