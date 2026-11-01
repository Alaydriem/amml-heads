use std::sync::Arc;

use heads_core::services::{CellFlags, ChatHeadPlacer, GlyphCells, GlyphPage, PlayerDirectory, UploadBackoff};
use heads_core::structs::{ChatLine, FaceImage};

/// What chat heads knows: the cell of each player, the names to find in a line and the glyph page that holds the
/// faces. Bounded by `GlyphCells::CAPACITY` players for the life of the process.
pub struct ChatState {
    cells: GlyphCells,
    directory: PlayerDirectory,
    page: GlyphPage,
    chat: bool,
    active: bool,
    page_uploaded: bool,
    backoff: UploadBackoff,
}

impl ChatState {
    /// The vanilla glyph row 0 (16 pixel rows of 256 RGBA pixels) that the page starts with.
    pub const VANILLA_ROW0: &'static [u8] = include_bytes!("../../assets/glyph_e1_row0.rgba");

    pub fn new(page: GlyphPage, flags: Arc<CellFlags>) -> Self {
        Self {
            cells: GlyphCells::new(flags),
            directory: PlayerDirectory::new(),
            page,
            chat: true,
            active: true,
            page_uploaded: false,
            backoff: UploadBackoff::new(),
        }
    }

    pub fn set_chat(&mut self, chat: bool) {
        self.chat = chat;
    }

    /// Stops the state reading packets and lines while the mod is off. Cells and names are kept.
    pub fn suspend(&mut self) {
        self.active = false;
    }

    /// Takes packets and lines again, and offers the page for upload again.
    pub fn resume(&mut self) {
        self.active = true;
        self.page.mark_dirty();
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Keeps the identity of a player seen in a player list, so a name or XUID it shares with another player gives
    /// no head. Called for every Add entry, before the face is read.
    pub fn on_identity(&mut self, uuid: [u8; 16], xuid: &str, name: &str) {
        if self.active {
            self.directory.note(uuid, xuid, name);
        }
    }

    /// Offers the page for upload again at once, after the world was left: a resource reload may have put the
    /// vanilla page back on the font.
    pub fn on_world_exit(&mut self) {
        self.page.mark_dirty();
        self.backoff.reset();
    }

    /// Gives the player a cell, records the names and draws the face into the page.
    pub fn on_face(&mut self, uuid: [u8; 16], xuid: &str, name: &str, face: &FaceImage) {
        if !self.active {
            return;
        }
        let Some(cell) = self.cells.cell_for(uuid) else {
            if self.cells.full_reported() {
                curia::warn!("chat head cells are full", { cells: GlyphCells::CAPACITY });
            }
            return;
        };
        if !self.directory.record(uuid, xuid, name) {
            return;
        }
        self.page.write_face(cell, face);
        self.cells.mark_face(cell);
    }

    /// Redraws the face of a player that has a cell. A player without one is ignored.
    pub fn on_skin(&mut self, uuid: [u8; 16], face: &FaceImage) {
        if !self.active {
            return;
        }
        let Some(cell) = self.cells.cell_of(uuid) else { return };
        self.page.write_face(cell, face);
        self.cells.mark_face(cell);
    }

    /// The page pixels when they changed since the last call.
    pub fn take_page(&mut self) -> Option<&[u8]> {
        self.page.take_dirty()
    }

    /// Hands the changed page to `upload`, called once a frame. A page that did not upload is offered again on a
    /// later call, as `UploadBackoff` spaces the tries.
    pub fn flush_page(&mut self, upload: impl FnOnce(&[u8]) -> bool) {
        if !self.active || !self.backoff.ready() {
            return;
        }
        let Some(page) = self.page.take_dirty() else { return };
        if !upload(page) {
            self.page.mark_dirty();
            if self.backoff.failed() {
                curia::warn!("chat glyph page upload failed", {
                    failures_before_backoff: UploadBackoff::FAILURES_BEFORE_BACKOFF,
                    retry_frames: UploadBackoff::RETRY_FRAMES
                });
            }
            return;
        }
        self.backoff.reset();
        if !self.page_uploaded {
            self.page_uploaded = true;
            curia::info!("chat glyph page uploaded");
        }
    }

    /// The line as it must be shown, or `None` when it stays as it is or the state is suspended. Face code points
    /// already in the line are always removed. The face glyph is put in only while the `chat` setting is on and
    /// after the page has uploaded once.
    pub fn glyph_line(&self, line: &ChatLine) -> Option<String> {
        if !self.active {
            return None;
        }
        if !self.chat || !self.page_uploaded {
            return ChatHeadPlacer::clean(line.full);
        }
        ChatHeadPlacer::edit(line, &self.cells, &self.directory)
    }
}
