use bevy::{prelude::*, ui::RelativeCursorPosition};

use game_state::prelude::*;

use super::*;

/// Plugin which updates the `EditorCursorStatus` resource with the cursor state.
///
/// The cursor status will be set to `EditorCursorState::Blocked` when blocked by the UI, `EditorCursorStatus::Active`
/// when it is not blocked.
pub struct EditorCursorStatusPlugin;

impl Plugin for EditorCursorStatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_cursor_status.in_set(EditorCursorSet::Status),
                setup_new_pickable_blocks,
            ),
        );
    }
}

/// Initializes new `PickableBlock` nodes by making sure they have a `RelativeCursorPosition` component. We use the
/// `RelativeCursorPosition` component to determine whether the node has mouse focus.
fn setup_new_pickable_blocks(
    mut commands: Commands,
    query: Query<Entity, (Added<PickableBlock>, Without<RelativeCursorPosition>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(RelativeCursorPosition::default());
    }
}

/// Updates the `EditorCursorStatus` resource each frame by checking if any blocking nodes have mouse focus.
fn update_cursor_status(
    block_query: Query<&RelativeCursorPosition, With<PickableBlock>>,
    mut cursor_status: ResMut<EditorCursorStatus>,
) {
    let block = block_query
        .iter()
        .find(|cursor_pos| cursor_pos.mouse_over());
    let new_status = if block.is_some() {
        EditorCursorStatus::Blocked
    } else {
        EditorCursorStatus::Active
    };

    if new_status != *cursor_status {
        *cursor_status = new_status;
    }
}
