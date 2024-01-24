use bevy::prelude::*;

use super::ToolButtonPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            //     .init_resource(GameUiStyle)
            .add_plugins(ToolButtonPlugin);
    }
}
