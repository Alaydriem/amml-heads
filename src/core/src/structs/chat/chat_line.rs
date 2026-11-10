use crate::structs::chat::MessageKind;

/// A chat line as the game gives it: the full text, the author's name and the author's XUID.
#[derive(Clone, Copy, Debug)]
pub struct ChatLine<'a> {
    pub kind: MessageKind,
    pub full: &'a str,
    pub username: &'a str,
    pub author_xuid: &'a str,
}
