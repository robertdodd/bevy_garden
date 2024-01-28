use bevy::prelude::*;

use super::ToolButtonPlugin;

/// Plugin that adds UI widgets to the game.
pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToolButtonPlugin);
    }
}
