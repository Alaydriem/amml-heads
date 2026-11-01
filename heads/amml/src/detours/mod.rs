// The game functions the mod hooks. The packet handlers, the client update and the two chat detours are
// amml_detour! bodies; the two draw detours are written by hand because the render original must run between the
// tagging and the restore.

pub mod char_width;
pub mod client_update;
pub mod draw_icon;
pub mod exit_level;
pub mod gui_message;
pub mod player_list;
pub mod player_skin;
pub mod render;
