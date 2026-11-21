use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError, TryLockError};

use amm_client::link::LinkState;
use amml_api::amml_abi::{ChatMessageAdded, LocatorBarIconDraw, PlayerListUpdate, PlayerSkinChanged, TexturesReloaded};
use amml_api::{
    Chat, ChatMessageAddedExt, LocatorBarIconDrawExt, Logger, PlayerListAction, PlayerListEntry, PlayerListUpdateExt,
    PlayerSkinChangedExt, Rect, Rgba, TextureId, TextureReload, Textures, TexturesReloadedExt,
};
use heads_core::config::HeadsSettings;
use heads_core::services::{IconSampler, QuadScaler};
use heads_core::structs::{ChatLine, FaceImage, FaceKey, FaceUpdate, IconRect, MessageKind};

use crate::services::{ChatBoard, FaceBoard, FaceReader, HeadsLink, SkinDiagnostics};
use crate::structs::{LineBudget, SettingsCell};

// what the listeners share: the faces and their textures, the chat icons, the settings and the manager link.
//
// Network events (`on_player_list`, `on_player_skin`) can run on the packet handler thread: they copy what they need
// out of the lent views before they return, and call only the loader calls that are allowed on any thread. The
// icon draw and chat line listeners never wait for a lock a network event holds. The tick and the icon calls run on
// the game thread. No two locks are held at once.
pub struct HeadsRuntime {
    log: Logger,
    textures: Textures,
    chat: Chat,
    settings: SettingsCell,
    faces: Mutex<FaceBoard>,
    chat_board: Mutex<ChatBoard>,
    link: Mutex<Option<HeadsLink>>,
    configured: AtomicBool,
    diagnostics: SkinDiagnostics,
    // ticks to wait before the next upload retry on the tick, after an upload the loader refused
    upload_wait: AtomicU32,
    // ticks to wait before the next icon retry, while the loader cannot take icons
    icon_wait: AtomicU32,
    upload_failed: AtomicBool,
    drawn: AtomicBool,
    ready_lines: LineBudget,
    uploaded_lines: LineBudget,
    icon_lines: LineBudget,
    edit_lines: LineBudget,
}

impl HeadsRuntime {
    // longest name or XUID the chat feed reads; a longer one gives no chat head
    const MAX_IDENTITY: usize = 64;
    // most icons registered in one tick
    const ICONS_PER_TICK: usize = 16;
    // ticks between two tries while a loader call is refused
    const RETRY_TICKS: u32 = 60;

    pub fn new(log: Logger, textures: Textures, chat: Chat) -> Self {
        Self {
            log,
            textures,
            chat,
            settings: SettingsCell::new(HeadsSettings::default()),
            faces: Mutex::new(FaceBoard::new()),
            chat_board: Mutex::new(ChatBoard::new()),
            link: Mutex::new(None),
            configured: AtomicBool::new(false),
            diagnostics: SkinDiagnostics::new(log),
            upload_wait: AtomicU32::new(0),
            icon_wait: AtomicU32::new(0),
            upload_failed: AtomicBool::new(false),
            drawn: AtomicBool::new(false),
            ready_lines: LineBudget::new(16),
            uploaded_lines: LineBudget::new(8),
            icon_lines: LineBudget::new(16),
            edit_lines: LineBudget::new(16),
        }
    }

    pub fn settings(&self) -> &SettingsCell {
        &self.settings
    }

    pub fn face_count(&self) -> usize {
        self.lock_faces().len()
    }

    pub fn icon_count(&self) -> usize {
        self.lock_chat().icons()
    }

    // enable: the settings come from this link once it is connected; without one the defaults stay
    pub fn attach(&self, link: Option<HeadsLink>) {
        let mut slot = self.link.lock().unwrap_or_else(PoisonError::into_inner);
        *slot = link;
        self.configured.store(false, Ordering::Relaxed);
    }

    // disable: the link closes, the faces and the icons go
    pub fn detach(&self) {
        drop(self.link.lock().unwrap_or_else(PoisonError::into_inner).take());
        self.world_left();
    }

    // once per client tick on the game thread
    pub fn tick(&self) {
        self.poll_link();
        if !Self::wait(&self.upload_wait) {
            self.flush_uploads();
        }
        self.register_icons();
    }

    // a player list packet was handled
    pub fn on_player_list(&self, event: &PlayerListUpdate) {
        let action = event.action();
        let mut entries = 0usize;
        for entry in event.entries() {
            entries += 1;
            match action {
                PlayerListAction::Add => self.add_entry(&entry),
                PlayerListAction::Remove => self.apply_face(FaceUpdate::Remove { uuid: entry.uuid().0 }),
            }
        }
        self.flush_uploads();
        let action = if action == PlayerListAction::Add { "add" } else { "remove" };
        self.log.info("heads player list", &[("action", action), ("entries", &entries.to_string())]);
    }

    // a player skin packet was handled
    pub fn on_player_skin(&self, event: &PlayerSkinChanged) {
        let (Some(uuid), Some(skin)) = (event.uuid(), event.skin()) else { return };
        let uuid = uuid.0;
        let read = FaceReader::read(&skin);
        self.diagnostics.record("PlayerSkin", uuid, None, &skin, &read);
        let Some(face) = read.face else { return };
        self.log.info("heads skin changed", &[("uuid", &Self::short_id(&uuid)), ("source", read.source)]);
        let icon = Box::new(IconSampler::sample(&face));
        self.lock_chat().on_skin(uuid, icon);
        self.apply_face(FaceUpdate::Skin { uuid, face });
        self.flush_uploads();
    }

    // one waypoint icon of the locator bar is about to be drawn: a player with a face gets it in place of the dot
    pub fn icon(&self, event: &mut LocatorBarIconDraw) {
        if !self.settings.enabled() {
            return;
        }
        let Some(actor) = event.actor() else { return };
        let texture = match self.faces.try_lock() {
            Ok(mut faces) => faces.texture_for(actor.0),
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner().texture_for(actor.0),
            Err(TryLockError::WouldBlock) => None,
        };
        let Some(texture) = texture else { return };
        let rect = event.rect();
        let scaled = QuadScaler::scale(IconRect::new(rect.x, rect.y, rect.width, rect.height), self.settings.scale());
        event.replace_texture(texture);
        event.set_rect(Rect { x: scaled.x, y: scaled.y, width: scaled.w, height: scaled.h });
        event.set_color(Rgba { r: 255, g: 255, b: 255, a: 255 });
        self.note_drawn(actor.0, texture);
    }

    // a chat line was built: the face glyph of its sender goes before the name
    pub fn chat_line(&self, event: &ChatMessageAdded) {
        let kind = MessageKind::from_raw(i32::try_from(event.kind().0).unwrap_or(-1));
        let show = self.settings.chat() && event.inline_icons();
        let Some(full) = event.text() else { return };
        let line =
            ChatLine { kind, full, username: event.username().unwrap_or(""), author_xuid: event.xuid().unwrap_or("") };
        let (text, filtered) = {
            let board = match self.chat_board.try_lock() {
                Ok(board) => board,
                Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
                Err(TryLockError::WouldBlock) => return,
            };
            let filtered = event.filtered_text().and_then(|full| board.glyph_line(&ChatLine { full, ..line }, show));
            (board.glyph_line(&line, show), filtered)
        };
        let mut edited = false;
        if let Some(text) = text {
            edited |= event.set_text(&text).is_ok();
        }
        if let Some(filtered) = filtered {
            edited |= event.set_filtered_text(&filtered).is_ok();
        }
        if edited && self.edit_lines.take() {
            self.log.info("heads chat line edited", &[("kind", &event.kind().0.to_string())]);
        }
    }

    // the loader bound or freed the mod textures. After a device reset every texture id is stale, so each face is
    // uploaded again at once. After a resource pack change the ids stay valid and the loader keeps the pixels of an
    // upload, so no face is touched. The inline icons live on a page the loader owns and uploads again itself
    pub fn textures_reloaded(&self, event: &TexturesReloaded) {
        let reason = event.reason();
        let (name, uploaded) = match reason {
            TextureReload::DeviceReset => {
                self.lock_faces().invalidate_textures();
                self.upload_wait.store(0, Ordering::Relaxed);
                ("device_reset", self.flush_uploads())
            }
            TextureReload::ResourcePack => ("resource_pack", 0),
            TextureReload::Other => ("other", 0),
        };
        self.log.info("heads textures reloaded", &[("reason", name), ("reuploaded", &uploaded.to_string())]);
    }

    // the world is being left: every face, texture and icon goes
    pub fn world_left(&self) {
        self.apply_face(FaceUpdate::Clear);
        self.lock_chat().clear();
        self.log.info("heads world left", &[("faces", &self.face_count().to_string())]);
    }

    fn add_entry(&self, entry: &PlayerListEntry<'_>) {
        let Some(actor) = entry.actor_id() else { return };
        let uuid = entry.uuid().0;
        let identity = Self::identity(entry);
        if let Some((name, xuid)) = &identity {
            self.lock_chat().on_identity(uuid, xuid, name);
        }
        let Some(skin) = entry.skin() else { return };
        let read = FaceReader::read(&skin);
        self.diagnostics.record("PlayerList.Add", uuid, Some(actor.0), &skin, &read);
        let Some(face) = read.face else { return };
        if self.ready_lines.take() {
            self.log.info("heads face ready", &[("source", read.source), ("head", read.head_source)]);
        }
        if let Some((name, xuid)) = &identity {
            let icon = Box::new(IconSampler::sample(&face));
            let mut board = self.lock_chat();
            board.on_face(uuid, xuid, name, icon);
            if board.full_reported() {
                self.log.warn("heads chat head icons are full", &[]);
            }
        }
        self.apply_face(FaceUpdate::Add { uuid, actor: actor.0, face });
    }

    // the name and XUID of an Add entry, or `None` when one is missing or too long
    fn identity(entry: &PlayerListEntry<'_>) -> Option<(String, String)> {
        let (name, xuid) = (entry.name()?, entry.xuid()?);
        (name.len() <= Self::MAX_IDENTITY && xuid.len() <= Self::MAX_IDENTITY)
            .then(|| (name.to_owned(), xuid.to_owned()))
    }

    // one update to the faces; the textures it frees are unloaded after the lock is given up
    fn apply_face(&self, update: FaceUpdate) {
        let released = {
            let mut faces = self.lock_faces();
            faces.apply(update);
            faces.take_released()
        };
        self.unload(released);
    }

    // uploads the faces that still wait for a texture
    fn flush_uploads(&self) -> usize {
        let mut uploaded = 0;
        if !self.settings.enabled() {
            return uploaded;
        }
        let mut jobs: Vec<(FaceKey, Box<FaceImage>)> = Vec::new();
        {
            let mut faces = self.lock_faces();
            if !faces.has_pending_upload() {
                return uploaded;
            }
            faces.take_pending(&mut jobs);
        }
        let mut refused = false;
        for (key, face) in jobs {
            if refused {
                self.lock_faces().retry_upload(key);
                continue;
            }
            match self.textures.upload_rgba(FaceImage::SIDE, FaceImage::SIDE, &face.pixels) {
                Ok(texture) => {
                    self.upload_failed.store(false, Ordering::Relaxed);
                    uploaded += 1;
                    self.store_texture(key, texture);
                }
                Err(error) => {
                    refused = true;
                    self.lock_faces().retry_upload(key);
                    if !self.upload_failed.swap(true, Ordering::Relaxed) {
                        self.log.warn("heads face upload refused", &[("error", &error.to_string())]);
                    }
                    self.upload_wait.store(Self::RETRY_TICKS, Ordering::Relaxed);
                }
            }
        }
        uploaded
    }

    fn store_texture(&self, key: FaceKey, texture: TextureId) {
        let stale = self.lock_faces().attach(key, texture);
        if self.uploaded_lines.take() {
            self.log.info("heads face uploaded", &[("actor", &key.actor.to_string())]);
        }
        if let Some(stale) = stale {
            self.unload(vec![stale]);
        }
    }

    fn unload(&self, textures: Vec<TextureId>) {
        for texture in textures {
            // a texture of a world that was left is stale already: the loader freed it
            let _ = self.textures.unload(texture);
        }
    }

    // registers the icons that wait for the game thread
    fn register_icons(&self) {
        let (done, full) = {
            let mut board = self.lock_chat();
            if board.waiting() == 0 || Self::wait(&self.icon_wait) {
                return;
            }
            let done = board.register_waiting(&self.chat, Self::ICONS_PER_TICK);
            if board.waiting() > 0 && done.len() < Self::ICONS_PER_TICK {
                self.icon_wait.store(Self::RETRY_TICKS, Ordering::Relaxed);
            }
            (done, board.full_reported())
        };
        for (glyph, new) in done {
            if new && self.icon_lines.take() {
                self.log.info("heads chat icon registered", &[("glyph", &format!("U+{:04X}", glyph as u32))]);
            }
        }
        if full {
            self.log.warn("heads chat head icons are full", &[]);
        }
    }

    // reads the settings once the link is connected, then the manager events
    fn poll_link(&self) {
        let mut slot = self.link.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(link) = slot.as_mut() else { return };
        let connected = link.state() == LinkState::Connected;
        let mut reload = connected && !self.configured.swap(true, Ordering::Relaxed);
        reload |= link.poll() && connected;
        if reload {
            let settings = link.settings();
            self.settings.store(settings);
            self.log.info(
                "heads settings applied",
                &[
                    ("enabled", &settings.enabled.to_string()),
                    ("scale", &settings.scale().to_string()),
                    ("chat", &settings.chat.to_string()),
                ],
            );
        }
    }

    fn note_drawn(&self, actor: i64, texture: TextureId) {
        if !self.drawn.load(Ordering::Relaxed)
            && self.textures.ready(texture)
            && !self.drawn.swap(true, Ordering::Relaxed)
        {
            self.log.info("heads icon face drawn", &[("actor", &actor.to_string())]);
        }
    }

    // true while ticks are left to wait; spends one
    fn wait(ticks: &AtomicU32) -> bool {
        ticks.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |left| left.checked_sub(1)).is_ok()
    }

    fn lock_faces(&self) -> MutexGuard<'_, FaceBoard> {
        self.faces.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn lock_chat(&self) -> MutexGuard<'_, ChatBoard> {
        self.chat_board.lock().unwrap_or_else(PoisonError::into_inner)
    }

    // the first four bytes of the uuid as 8 hex digits
    fn short_id(uuid: &[u8; 16]) -> String {
        uuid[..4].iter().map(|byte| format!("{byte:02x}")).collect()
    }
}
