use crate::services::{GlyphCells, PlayerDirectory};
use crate::structs::{ChatLine, GlyphCell, MessageKind};

/// Decides which player a chat line belongs to and where the face glyph goes in it. When the rules
/// are not sure, the answer is `None`. Only `edit`, `insert` and `clean` allocate.
pub struct ChatHeadPlacer;

impl ChatHeadPlacer {
    const SECTION_SIGN: [u8; 2] = [0xC2, 0xA7];
    // the first UTF-8 byte of every code point from U+E000 to U+EFFF, which holds the face cells
    const FACE_LEAD_BYTE: u8 = 0xEE;

    pub fn place(line: &ChatLine, cells: &GlyphCells, directory: &PlayerDirectory) -> Option<usize> {
        let (uuid, offset) = Self::resolve(line, directory)?;
        Self::glyph_of(uuid, cells)?;
        Some(offset)
    }

    pub fn glyph_for(line: &ChatLine, cells: &GlyphCells, directory: &PlayerDirectory) -> Option<char> {
        let (uuid, _) = Self::resolve(line, directory)?;
        Self::glyph_of(uuid, cells)
    }

    /// The line as it must be shown: every face code point already in it removed, and the one face glyph the
    /// rules chose put in. `None` when that is the line as it is, so nothing is allocated.
    pub fn edit(line: &ChatLine, cells: &GlyphCells, directory: &PlayerDirectory) -> Option<String> {
        let placed = Self::resolve(line, directory).and_then(|(uuid, at)| Some((at, Self::glyph_of(uuid, cells)?)));
        match placed {
            Some((at, glyph)) => Some(Self::insert(line.full, at, glyph)),
            None => Self::clean(line.full),
        }
    }

    /// `full` with `glyph` at byte `at`, and every face code point that was in `full` removed.
    pub fn insert(full: &str, at: usize, glyph: char) -> String {
        let mut out = String::with_capacity(full.len() + glyph.len_utf8());
        Self::push_clean(&mut out, &full[..at]);
        out.push(glyph);
        Self::push_clean(&mut out, &full[at..]);
        out
    }

    /// `full` with every face code point removed, or `None` when it holds none.
    pub fn clean(full: &str) -> Option<String> {
        if !Self::holds_face_codepoint(full) {
            return None;
        }
        let mut out = String::with_capacity(full.len());
        Self::push_clean(&mut out, full);
        Some(out)
    }

    fn holds_face_codepoint(text: &str) -> bool {
        text.as_bytes().contains(&Self::FACE_LEAD_BYTE) && text.chars().any(Self::is_face_codepoint)
    }

    fn push_clean(out: &mut String, text: &str) {
        if !Self::holds_face_codepoint(text) {
            out.push_str(text);
            return;
        }
        out.extend(text.chars().filter(|c| !Self::is_face_codepoint(*c)));
    }

    fn is_face_codepoint(c: char) -> bool {
        GlyphCell::from_codepoint(c as u32).is_some()
    }

    fn glyph_of(uuid: [u8; 16], cells: &GlyphCells) -> Option<char> {
        let glyph = cells.cell_of(uuid)?.codepoint();
        cells.has_face(glyph as u32).then_some(glyph)
    }

    fn resolve(line: &ChatLine, directory: &PlayerDirectory) -> Option<([u8; 16], usize)> {
        match line.kind {
            MessageKind::Chat if line.username.is_empty() => Self::resolve_named(line.full, directory),
            MessageKind::Chat | MessageKind::Whisper => {
                let uuid = if line.author_xuid.is_empty() {
                    directory.by_name(line.username)?
                } else {
                    directory.by_xuid(line.author_xuid)?
                };
                Some((uuid, 0))
            }
            MessageKind::Localized => Self::resolve_named(line.full, directory),
            MessageKind::Other => None,
        }
    }

    fn resolve_named(full: &str, directory: &PlayerDirectory) -> Option<([u8; 16], usize)> {
        let anchor = Self::anchor(full);
        let mut found: Option<&str> = None;
        let mut elsewhere = false;
        for (name, _) in directory.names() {
            match Self::sole_word_match(full, name) {
                Some(at) if at == anchor => {
                    if found.is_some() {
                        return None;
                    }
                    found = Some(name);
                }
                Some(_) => elsewhere = true,
                None => {}
            }
        }
        if elsewhere {
            return None;
        }
        let name = found?;
        Some((directory.by_name(name)?, anchor))
    }

    fn anchor(full: &str) -> usize {
        let mut at = 0;
        while let Some(rest) = full.get(at..) {
            if !rest.as_bytes().starts_with(&Self::SECTION_SIGN) {
                break;
            }
            let Some(code) = rest[Self::SECTION_SIGN.len()..].chars().next() else {
                break;
            };
            at += Self::SECTION_SIGN.len() + code.len_utf8();
        }
        if full.get(at..).is_some_and(|rest| rest.starts_with("* ")) {
            at += 2;
        }
        at
    }

    fn sole_word_match(full: &str, name: &str) -> Option<usize> {
        if name.is_empty() {
            return None;
        }
        let mut matches = full.match_indices(name);
        let (start, _) = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        let bytes = full.as_bytes();
        let after_ok = bytes.get(start + name.len()).is_none_or(|b| !Self::is_word_byte(*b));
        (Self::before_ok(bytes, start) && after_ok).then_some(start)
    }

    fn before_ok(bytes: &[u8], start: usize) -> bool {
        if start >= 3 && bytes[start - 3..start - 1] == Self::SECTION_SIGN && bytes[start - 1].is_ascii() {
            return true;
        }
        start == 0 || !Self::is_word_byte(bytes[start - 1])
    }

    fn is_word_byte(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_'
    }
}
