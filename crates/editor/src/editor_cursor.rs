use bevy::{prelude::*, ui::RelativeCursorPosition};

use game_state::prelude::*;

/// Plugin which handles editor cursors
pub struct EditorCursorPlugin;

impl Plugin for EditorCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorCursorStatus>()
            .configure_sets(Update, EditorCursorSet.run_if(in_game))
            .add_systems(
                Update,
                (
                    update_cursor_status.in_set(EditorCursorSet),
                    setup_new_pickable_blocks,
                    handle_pickable_block_removed.run_if(any_component_removed::<PickableBlock>()),
                ),
            );
    }
}

#[derive(Resource, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum EditorCursorStatus {
    #[default]
    Blocked,
    Active,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct EditorCursorSet;

/// Run condition that returns true if the cursor is not blocked by a `PickableBlock` component.
pub fn cursor_not_blocked(cursor_status: Res<EditorCursorStatus>) -> bool {
    *cursor_status != EditorCursorStatus::Blocked
}

/// initializes new PickableBlock nodes by making sure they have a `RelativeCursorPosition` component
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

/// System that removes the `RelativeCursorPosition` component from entities when the `PickableBlock` component is
/// removed.
fn handle_pickable_block_removed(
    mut removed: RemovedComponents<PickableBlock>,
    mut commands: Commands,
    query: Query<(), (Without<PickableBlock>, With<RelativeCursorPosition>)>,
) {
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).remove::<RelativeCursorPosition>();
        }
    }
}

/// Updates the status of the cursor if its blocked or active
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
