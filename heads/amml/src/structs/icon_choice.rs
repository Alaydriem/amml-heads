use crate::layouts::TexturePtrLayout;

/// What the icon blit should draw for one packed colour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconChoice {
    /// Draw this face texture, in white.
    Face(*const TexturePtrLayout),
    /// Draw the vanilla dot in this packed colour.
    Vanilla(u32),
}
