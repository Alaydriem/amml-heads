#![allow(unsafe_code)]

use std::ffi::c_void;

use amml_msvc::{MsvcHeap, SharedPtr};
use crossbeam_channel::Receiver;
use heads_core::config::HeadsSettings;
use heads_core::services::{FaceStore, TagCodec};
use heads_core::structs::FaceUpdate;

use super::{MemoryReader, TextureBridge, WaypointTagger};
use crate::structs::{ClientAccess, GameTexture, IconChoice, TagFrame};

type GetLocalPlayerFn = unsafe extern "C" fn(usize) -> usize;
type GetTextureGroupFn = unsafe extern "C" fn(usize, *mut SharedPtr<c_void>) -> *mut SharedPtr<c_void>;

/// Everything the two draw detours share: the faces, the textures they own and the frame being drawn.
///
/// Updates from the packet side are applied only at the start of a frame, before the waypoints are tagged, so a
/// texture is never released while the frame can still draw it. Released textures wait in `deferred` and are
/// unloaded when the frame ends.
pub struct RenderState {
    store: FaceStore<GameTexture>,
    receiver: Receiver<FaceUpdate>,
    frame: Box<TagFrame>,
    settings: HeadsSettings,
    bridge: TextureBridge,
    access: ClientAccess,
    deferred: Vec<GameTexture>,
    released: Vec<GameTexture>,
    client: usize,
    group: Option<SharedPtr<c_void>>,
    heap: Option<MsvcHeap>,
    reset_pending: bool,
    uploaded: u64,
    decode_misses: u64,
    logged_uploaded: u64,
}

// SAFETY: the state holds plain heap pointers into the game and no thread-affine data. It is used only from the
// game render thread; the lock around it in `RenderRuntime` keeps access exclusive.
unsafe impl Send for RenderState {}

impl RenderState {
    // most updates applied in one frame
    const DRAIN_PER_FRAME: usize = 64;
    // most textures waiting for the end of the frame; the name pool already bounds this
    const DEFERRED_CAPACITY: usize = 256;

    pub fn new(
        store: FaceStore<GameTexture>,
        receiver: Receiver<FaceUpdate>,
        settings: HeadsSettings,
        bridge: TextureBridge,
        access: ClientAccess,
        heap: Option<MsvcHeap>,
    ) -> Self {
        Self {
            store,
            receiver,
            frame: Box::new(TagFrame::new()),
            settings,
            bridge,
            access,
            deferred: Vec::with_capacity(Self::DEFERRED_CAPACITY),
            released: Vec::with_capacity(Self::DRAIN_PER_FRAME * 2),
            client: 0,
            group: None,
            heap,
            reset_pending: false,
            uploaded: 0,
            decode_misses: 0,
            logged_uploaded: 0,
        }
    }

    pub fn set_settings(&mut self, settings: HeadsSettings) {
        self.settings = settings;
    }

    /// The face scale to draw with.
    pub fn scale(&self) -> f32 {
        self.settings.scale()
    }

    /// How many faces the store holds.
    pub fn face_count(&self) -> usize {
        self.store.len()
    }

    /// How many resource locations the texture bridge keeps.
    pub fn locations_built(&self) -> usize {
        self.bridge.locations_built()
    }

    /// Drops the queued updates and clears every face, as if the world was left. Used when the channel was too
    /// full to take a `Clear`. The textures are unloaded when the next frame ends.
    pub fn request_clear(&mut self) {
        for _ in 0..self.receiver.len() {
            if self.receiver.try_recv().is_err() {
                break;
            }
        }
        self.apply(FaceUpdate::Clear);
    }

    /// How many textures wait for the end of the frame.
    pub fn deferred_len(&self) -> usize {
        self.deferred.len()
    }

    /// Whether the waypoint colours are tagged right now.
    pub fn is_tagged(&self) -> bool {
        self.frame.is_active()
    }

    /// Starts a frame: applies the queued updates, uploads pending faces and tags the waypoints.
    pub fn before_render(&mut self, client: usize) {
        self.client = client;
        self.drain();
        if !self.settings.enabled {
            return;
        }
        self.upload_pending();
        self.tag_waypoints();
    }

    /// Uploads the chat glyph page outside a render frame, with the texture group of `client`. The group is released
    /// again unless a frame holds it.
    pub fn upload_chat_page(&mut self, client: usize, page: &[u8]) -> bool {
        let held = self.group.is_some();
        let previous = self.client;
        self.client = client;
        let group = self.group_address();
        let uploaded = group != 0 && self.bridge.upload_page(group, page);
        if !held {
            self.release_group();
        }
        self.client = previous;
        uploaded
    }

    /// Ends a frame: puts the waypoint colours back and unloads the textures released this frame.
    pub fn after_render(&mut self) {
        // SAFETY: the waypoints tagged in `before_render` are alive; the render thread cannot have freed them.
        unsafe { WaypointTagger::restore(&mut self.frame, &self.access.waypoints) };
        self.unload_deferred();
        self.release_group();
        self.drop_locations_when_idle();
        self.trace_uploads();
    }

    /// What the icon blit should draw for a packed colour.
    pub fn icon_for(&mut self, packed: u32) -> IconChoice {
        if !self.frame.is_active() {
            return IconChoice::Vanilla(packed);
        }
        let Some(saved) = TagCodec::decode(packed).and_then(|index| self.frame.get(usize::from(index))) else {
            self.note_decode_miss();
            return IconChoice::Vanilla(packed);
        };
        let face = saved.actor().and_then(|actor| self.store.texture_for(actor)).map(GameTexture::texture_ptr);
        match face {
            Some(ptr) => IconChoice::Face(ptr),
            None => IconChoice::Vanilla(saved.vanilla_packed()),
        }
    }

    // a colour in a tagged frame that is not a tag; every visible waypoint is tagged, so each one is worth a line.
    // Logged at 1, 2, 4, 8, ... so a bad frame cannot flood the log
    fn note_decode_miss(&mut self) {
        self.decode_misses += 1;
        if self.decode_misses.is_power_of_two() {
            curia::trace!("a colour in a tagged frame did not decode", { misses: self.decode_misses });
        }
    }

    // one line per frame in which faces were uploaded
    fn trace_uploads(&mut self) {
        if self.uploaded != self.logged_uploaded {
            self.logged_uploaded = self.uploaded;
            curia::trace!("faces uploaded", { total: self.uploaded });
        }
    }

    fn drain(&mut self) {
        for _ in 0..Self::DRAIN_PER_FRAME {
            let Ok(update) = self.receiver.try_recv() else { break };
            self.apply(update);
        }
    }

    fn apply(&mut self, update: FaceUpdate) {
        if matches!(update, FaceUpdate::Clear) {
            self.reset_pending = true;
        }
        self.store.apply(update, &mut self.released);
        while let Some(texture) = self.released.pop() {
            self.defer(texture);
        }
    }

    // After the world is left and every texture is unloaded, the resource locations are freed too; they are built
    // again on the next upload.
    fn drop_locations_when_idle(&mut self) {
        if !self.reset_pending || !self.deferred.is_empty() || self.bridge.free_names() != TextureBridge::POOL {
            return;
        }
        if let Some(heap) = &self.heap {
            self.bridge.shutdown(heap);
        }
        self.reset_pending = false;
    }

    fn defer(&mut self, texture: GameTexture) {
        if self.deferred.len() < Self::DEFERRED_CAPACITY {
            self.deferred.push(texture);
        } else {
            self.bridge.forget(texture);
        }
    }

    // Unloads the textures released since the last unload. Nothing can draw them now: it runs after the updates are
    // applied and before the waypoints are tagged, or after the frame is drawn.
    fn unload_deferred(&mut self) {
        if self.deferred.is_empty() {
            return;
        }
        let group = self.group_address();
        if group != 0 {
            for texture in self.deferred.drain(..) {
                self.bridge.unload(group, texture);
            }
        }
    }

    fn upload_pending(&mut self) {
        self.store.begin_frame();
        if !self.store.has_pending_upload() {
            return;
        }
        // names of released textures go back to the pool first, so a face that replaces another gets one
        self.unload_deferred();
        let group = self.group_address();
        if group == 0 {
            return;
        }
        while let Some((key, face)) = self.store.next_upload() {
            let Some(texture) = self.bridge.upload(group, face) else {
                // no name or no texture: the face waits for the next frame
                self.store.retry_upload(key);
                break;
            };
            self.uploaded += 1;
            if let Some(stale) = self.store.attach(key, texture) {
                self.defer(stale);
            }
        }
    }

    fn tag_waypoints(&mut self) {
        let Some(get_local_player) = self.client_function(self.access.get_local_player_slot) else { return };
        // SAFETY: `get_local_player` is the code in the getLocalPlayer slot of the live client's vtable.
        let player = unsafe { std::mem::transmute::<usize, GetLocalPlayerFn>(get_local_player)(self.client) };
        if player == 0 {
            return;
        }
        let head_at = player + self.access.locator_bar_offset + self.access.waypoints.list_head;
        // SAFETY: the player is live; the waypoint list head pointer is inside its client locator bar.
        let Some(head) = (unsafe { MemoryReader::read::<usize>(head_at) }) else {
            return;
        };
        // SAFETY: the list belongs to the live local player and is only changed on this thread.
        unsafe { WaypointTagger::tag(head, self.access.entity_vtable, &self.access.waypoints, &mut self.frame) };
    }

    // The texture group address for this frame, or 0 when the client has none. The shared_ptr is held until the
    // frame ends and released in `release_group`.
    fn group_address(&mut self) -> usize {
        if self.group.is_none() && self.client != 0 {
            // SAFETY: an all-zero shared_ptr is the empty pointer.
            let mut out: SharedPtr<c_void> = unsafe { std::mem::zeroed() };
            if let Some(get_texture_group) = self.client_function(self.access.get_texture_group_slot) {
                // SAFETY: `get_texture_group` is the code in the getTextureGroup slot of the live client's vtable;
                // it writes a shared_ptr through the hidden return pointer.
                unsafe { std::mem::transmute::<usize, GetTextureGroupFn>(get_texture_group)(self.client, &mut out) };
            }
            self.group = Some(out);
        }
        self.group.as_ref().and_then(SharedPtr::get).map_or(0, |group| std::ptr::from_ref(group) as usize)
    }

    // the code in a slot of the client's vtable
    fn client_function(&self, slot: usize) -> Option<usize> {
        // SAFETY: the client is live for the frame and its first word is its vtable.
        let vtable = unsafe { MemoryReader::read::<usize>(self.client)? };
        // SAFETY: the slot is inside the ClientInstance vtable, a table in the game image.
        let code = unsafe { MemoryReader::read::<usize>(vtable.checked_add(slot.checked_mul(8)?)?)? };
        (code != 0).then_some(code)
    }

    fn release_group(&mut self) {
        if let Some(mut group) = self.group.take() {
            // SAFETY: the pointer is empty or holds the use taken by getTextureGroup; it is released once.
            unsafe { group.release() };
        }
    }
}

impl Drop for RenderState {
    fn drop(&mut self) {
        // a disable during a tagged frame must not leave tag colours in the waypoints
        // SAFETY: the waypoints tagged in `before_render` are alive; the state is dropped on the render thread.
        unsafe { WaypointTagger::restore(&mut self.frame, &self.access.waypoints) };
        self.release_group();
    }
}
