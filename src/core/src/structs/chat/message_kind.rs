/// The kinds of chat message the placer tells apart. The raw values are those of the game's
/// `GuiMessage::MessageType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    Chat,
    Whisper,
    Localized,
    Other,
}

impl MessageKind {
    pub fn from_raw(raw: i32) -> MessageKind {
        match raw {
            1 => MessageKind::Chat,
            5 => MessageKind::Whisper,
            3 => MessageKind::Localized,
            _ => MessageKind::Other,
        }
    }
}
