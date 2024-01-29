pub const GAME_TITLE: &str = "Bevy Garden";

// Font Sizes
//
pub const FONT_SIZE_SM: f32 = 10.0;
pub const FONT_SIZE_BASE: f32 = 14.0;
pub const FONT_SIZE_LG: f32 = 16.0;

/// Save file location
///
/// The default filename to save levels to, relative to the assets directory: "assets/{SAVE_FILENAME}"
/// If it is a nested path (e.g. "level/level_1/level.scn.ron") then the folder structure will be created by the
/// `save` crate if doesn't exist.
pub const SAVE_FILENAME: &str = "level.scn.ron";
