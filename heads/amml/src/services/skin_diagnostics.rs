#![allow(unsafe_code)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};

use super::{MemoryReader, StdStringReader};
use crate::layouts::{ImageLayout, SerializedSkinLayout};
use crate::structs::{PacketOffsets, SkinEvent, SkinReport};
use heads_core::structs::UvRect;

/// Shows where a skin stops being a face: one log line per player for the first `LOG_LIMIT` players of a session.
pub struct SkinDiagnostics {
    // the players already logged; holds at most LOG_LIMIT entries
    seen: Mutex<Vec<[u8; 16]>>,
    logged: AtomicUsize,
}

impl SkinDiagnostics {
    pub const LOG_LIMIT: usize = 64;
    // longest string read from game memory, and most animated images listed
    const MAX_STRING: usize = 4096;
    const MAX_ANIMATED: usize = 16;

    pub fn new() -> Self {
        Self { seen: Mutex::new(Vec::with_capacity(Self::LOG_LIMIT)), logged: AtomicUsize::new(0) }
    }

    /// How many players were logged.
    pub fn logged(&self) -> usize {
        self.logged.load(Ordering::Relaxed)
    }

    /// Reads what the skin behind a `SerializedSkinRef` holds. `None` when a link is null.
    ///
    /// # Safety
    /// `skin_ref` must be null or the address of a live `shared_ptr` to a `SerializedSkinImpl` laid out as the
    /// 26.60 header says; the packet detour that holds the skin guarantees it for the call.
    pub unsafe fn inspect(skin_ref: usize, offsets: &PacketOffsets) -> Option<SkinReport> {
        // SAFETY: the caller guarantees `skin_ref` is null or a live shared_ptr, whose first word is the element.
        let element: usize = unsafe { MemoryReader::read(skin_ref)? };
        if element == 0 {
            return None;
        }
        // SAFETY: the element is a live SerializedSkinImpl and `skin_image` is inside it.
        let image: ImageLayout = unsafe { MemoryReader::read(element.checked_add(offsets.skin_image)?)? };
        let expected = u64::from(image.width) * u64::from(image.height) * 4;
        // SAFETY: the flags and strings are inside the live SerializedSkinImpl.
        let (is_premium, is_persona, default_geometry, resource_patch_len, animated_types) = unsafe {
            (
                MemoryReader::read::<u8>(element + SerializedSkinLayout::IS_PREMIUM).is_some_and(|flag| flag != 0),
                MemoryReader::read::<u8>(element + SerializedSkinLayout::IS_PERSONA).is_some_and(|flag| flag != 0),
                Self::string(element + SerializedSkinLayout::DEFAULT_GEOMETRY_NAME),
                Self::string_len(element + SerializedSkinLayout::RESOURCE_PATCH),
                Self::animated_types(element),
            )
        };
        Some(SkinReport {
            width: image.width,
            height: image.height,
            format: image.format,
            blob_size: image.blob_size,
            size_matches: expected == image.blob_size as u64,
            is_persona,
            is_premium,
            default_geometry,
            resource_patch_len,
            animated_types,
        })
    }

    /// Logs the skin of a player the first time it is seen, up to `LOG_LIMIT` players.
    ///
    /// # Safety
    /// As for `inspect`, for `event.skin_ref`.
    pub unsafe fn record(&self, event: &SkinEvent, offsets: &PacketOffsets) {
        if self.logged.load(Ordering::Relaxed) >= Self::LOG_LIMIT || !self.claim(event.uuid) {
            return;
        }
        // SAFETY: the caller guarantees the skin reference is live.
        let report = unsafe { Self::inspect(event.skin_ref, offsets) };
        let uuid = Self::short_id(&event.uuid);
        match report {
            Some(report) => curia::info!("skin read", {
                kind: event.kind,
                uuid: uuid.clone(),
                actor: event.actor,
                width: report.width,
                height: report.height,
                format: report.format,
                blob_size: report.blob_size,
                size_matches: report.size_matches,
                is_persona: report.is_persona,
                is_premium: report.is_premium,
                default_geometry: report.default_geometry,
                resource_patch_len: report.resource_patch_len,
                animated_images: report.animated_types.len(),
                animated_types: report.animated_types,
                outcome: event.outcome.clone(),
                head_source: event.head_source,
                face_rect: Self::rect(event.head.face),
                hat_rect: event.head.hat.map(Self::rect),
                geometry_json_len: event.geometry_json.map(str::len),
            }),
            None => curia::info!("skin read", {
                kind: event.kind,
                uuid: uuid.clone(),
                actor: event.actor,
                unreadable: true,
                outcome: event.outcome.clone(),
            }),
        }
    }

    // records the player; false when it was seen before or the log is full
    fn claim(&self, uuid: [u8; 16]) -> bool {
        let mut seen = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        if seen.len() >= Self::LOG_LIMIT || seen.contains(&uuid) {
            return false;
        }
        seen.push(uuid);
        self.logged.store(seen.len(), Ordering::Relaxed);
        true
    }

    fn rect(rect: UvRect) -> [f32; 4] {
        [rect.x, rect.y, rect.w, rect.h]
    }

    // the first four bytes of the uuid as 8 hex digits
    fn short_id(uuid: &[u8; 16]) -> String {
        uuid[..4].iter().map(|byte| format!("{byte:02x}")).collect()
    }

    // the text of a std::string member, or a marker when it does not look sane
    unsafe fn string(address: usize) -> String {
        // SAFETY: the caller passes the address of a std::string member of the live SerializedSkinImpl.
        unsafe { StdStringReader::text(address, Self::MAX_STRING) }.unwrap_or_else(|| "<invalid>".to_string())
    }

    // the length of a std::string, 0 when it does not look sane
    unsafe fn string_len(address: usize) -> usize {
        // SAFETY: as for `string`.
        unsafe { StdStringReader::size(address) }.unwrap_or(0)
    }

    // the mType of each element of mSkinAnimatedImages, at most MAX_ANIMATED of them
    unsafe fn animated_types(element: usize) -> Vec<u32> {
        // SAFETY: the vector is a member of the live SerializedSkinImpl.
        let Some([first, last]) =
            (unsafe { MemoryReader::read::<[usize; 2]>(element + SerializedSkinLayout::ANIMATED_IMAGES) })
        else {
            return Vec::new();
        };
        if first == 0 || last < first {
            return Vec::new();
        }
        let count = ((last - first) / SerializedSkinLayout::ANIMATED_IMAGE_SIZE).min(Self::MAX_ANIMATED);
        (0..count)
            .filter_map(|index| {
                let address = first
                    + index * SerializedSkinLayout::ANIMATED_IMAGE_SIZE
                    + SerializedSkinLayout::ANIMATED_IMAGE_TYPE;
                // SAFETY: the element lies inside the live vector.
                unsafe { MemoryReader::read::<u32>(address) }
            })
            .collect()
    }
}

impl Default for SkinDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}
