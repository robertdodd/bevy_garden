use bevy::prelude::*;

use game_state::prelude::*;

use super::*;

/// Plugin which handles editor cursors
pub struct EditorCursorPlugin;

impl Plugin for EditorCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorCursorStatus>()
            .configure_sets(
                Update,
                (
                    EditorCursorSet::Status.run_if(in_game),
                    EditorCursorSet::Transform.run_if(in_game),
                    EditorCursorSet::Click.run_if(in_game),
                )
                    .chain(),
            )
            .add_plugins((
                ObjectCursorPlugin,
                AttachableCursorPlugin,
                EditorCursorStatusPlugin,
            ));
    }
}
