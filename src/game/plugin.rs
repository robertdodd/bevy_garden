use bevy::prelude::*;

use super::{
    failed_to_load_menu::FailedToLoadMenuPlugin, new_level::NewLevelPlugin,
    tool_loader::ToolLoaderPlugin, tool_panel::ToolPanelPlugin,
};

/// Plugin responsible for in-game menus, ui, state transitions etc.
///
/// Basically, this plugin allows the player to interact with the game and triggers all the appropriate
/// state-transitions and events.
///
/// This is where you would add a main menu, settings menu, level-select screen, FPS counter, score UI etc.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ToolPanelPlugin,
            ToolLoaderPlugin,
            NewLevelPlugin,
            FailedToLoadMenuPlugin,
        ));
    }
}
