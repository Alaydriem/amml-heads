use std::collections::{HashMap, VecDeque};

use amml_api::{ApiError, Chat, ImageView, InlineIcon, PixelFormat};
use heads_core::services::{ChatHeadPlacer, FaceGlyphs, PlayerDirectory};
use heads_core::structs::{ChatLine, GlyphCell, IconImage};

use crate::structs::PendingIcon;

/// What chat heads knows: the players with a face, the names to find in a line and the inline icon of each face.
///
/// Bounds: at most `FaceGlyphs::CAPACITY` icons and `PlayerDirectory::CAPACITY` players until `clear`. An image waits
/// only for a recorded player and an image of the same player replaces the one waiting, so at most
/// `PlayerDirectory::CAPACITY` images wait for the game thread.
pub struct ChatBoard {
    directory: PlayerDirectory,
    glyphs: FaceGlyphs,
    icons: HashMap<[u8; 16], InlineIcon>,
    waiting: VecDeque<PendingIcon>,
}

impl ChatBoard {
    pub fn new() -> Self {
        Self {
            directory: PlayerDirectory::new(),
            glyphs: FaceGlyphs::new(),
            icons: HashMap::with_capacity(FaceGlyphs::CAPACITY),
            waiting: VecDeque::with_capacity(PlayerDirectory::CAPACITY),
        }
    }

    /// Keeps the identity of a player seen in a player list, so a name or XUID it shares with another player gives
    /// no head. Called for every Add entry, before the face is read.
    pub fn on_identity(&mut self, uuid: [u8; 16], xuid: &str, name: &str) {
        self.directory.note(uuid, xuid, name);
    }

    /// Records a player that has a face and queues the face as an icon. False when the directory is full.
    pub fn on_face(&mut self, uuid: [u8; 16], xuid: &str, name: &str, image: Box<IconImage>) -> bool {
        if !self.directory.record(uuid, xuid, name) {
            return false;
        }
        self.queue(PendingIcon { uuid, image });
        true
    }

    /// Queues the new face of a player that has one. A player that holds no icon is ignored.
    pub fn on_skin(&mut self, uuid: [u8; 16], image: Box<IconImage>) {
        if self.directory.knows(uuid) {
            self.queue(PendingIcon { uuid, image });
        }
    }

    pub fn waiting(&self) -> usize {
        self.waiting.len()
    }

    pub fn icons(&self) -> usize {
        self.icons.len()
    }

    pub fn glyph_of(&self, uuid: [u8; 16]) -> Option<char> {
        self.glyphs.glyph_of(uuid)
    }

    /// True once, after an icon was refused because every icon is in use.
    pub fn full_reported(&mut self) -> bool {
        self.glyphs.full_reported()
    }

    /// Registers or updates the icon of up to `limit` waiting images, on the game thread. Stops at the first image the
    /// loader cannot take now (`NotAvailable`: the default font is not a bitmap font yet) and keeps it and the rest
    /// for a later call. Gives the glyph of each icon made, and whether it is new.
    pub fn register_waiting(&mut self, chat: &Chat, limit: usize) -> Vec<(char, bool)> {
        let mut done = Vec::new();
        while done.len() < limit {
            let Some(job) = self.waiting.pop_front() else { break };
            match self.register_one(chat, &job) {
                Ok(made) => done.push(made),
                Err(ApiError::NotAvailable) => {
                    self.waiting.push_front(job);
                    break;
                }
                // all 240 icons are in use, or the loader refused the image: this player gets no head
                Err(_) => {}
            }
        }
        done
    }

    /// The line as it must be shown, or `None` when it stays as it is. Face code points already in the line are always
    /// removed. The face glyph is put in only when `show`.
    pub fn glyph_line(&self, line: &ChatLine, show: bool) -> Option<String> {
        if !show {
            return ChatHeadPlacer::clean(line.full);
        }
        ChatHeadPlacer::edit(line, &self.glyphs, &self.directory)
    }

    /// Forgets every player and frees every icon, as the world was left.
    pub fn clear(&mut self) {
        self.directory.clear();
        self.glyphs.clear();
        self.icons.clear();
        self.waiting.clear();
    }

    // an image for the same player replaces the one waiting
    fn queue(&mut self, job: PendingIcon) {
        if let Some(at) = self.waiting.iter().position(|waiting| waiting.uuid == job.uuid) {
            self.waiting.remove(at);
        }
        self.waiting.push_back(job);
    }

    fn register_one(&mut self, chat: &Chat, job: &PendingIcon) -> Result<(char, bool), ApiError> {
        let view = ImageView {
            width: IconImage::SIDE,
            height: IconImage::SIDE,
            format: PixelFormat::Rgba8,
            pixels: &job.image.pixels,
        };
        if let Some(icon) = self.icons.get(&job.uuid) {
            match icon.update(&view) {
                Ok(()) => return Ok((icon.glyph(), false)),
                // the world was left under the icon: register again
                Err(ApiError::Stale) => {
                    self.icons.remove(&job.uuid);
                }
                Err(error) => return Err(error),
            }
        }
        let icon = chat.register_inline_icon(&view, GlyphCell::WIDTH)?;
        let glyph = icon.glyph();
        if !self.glyphs.set(job.uuid, glyph) {
            return Err(ApiError::Refused);
        }
        self.icons.insert(job.uuid, icon);
        Ok((glyph, true))
    }
}

impl Default for ChatBoard {
    fn default() -> Self {
        Self::new()
    }
}
