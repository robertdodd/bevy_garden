use bevy::prelude::*;

use crate::prelude::{DeleteEventPlugin, EditorCursorPlugin, ToolLibrary, ToolStackPlugin};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToolLibrary>().add_plugins((
            ToolStackPlugin,
            DeleteEventPlugin,
            EditorCursorPlugin,
        ));
    }
}
