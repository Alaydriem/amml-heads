use heads_core::structs::IconImage;

/// An inline icon image waiting for the game thread: the icon of `uuid` is registered, or updated when it exists.
pub struct PendingIcon {
    pub uuid: [u8; 16],
    pub image: Box<IconImage>,
}
