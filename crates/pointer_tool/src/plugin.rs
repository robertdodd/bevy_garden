use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use bevy_helpers::generic_systems::{debug_state_changes, despawn_recursive_with};
use editor::prelude::*;
use game_effects::selected::*;
use game_state::prelude::*;

use crate::types::*;

/// Plugin which handles the "Pointer" tool.
///
/// This lets you select objects and delete them by pressing the "Delete" key.
pub struct PointerToolPlugin;

impl Plugin for PointerToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PointerToolState>()
            .add_systems(OnEnter(PointerToolState::Active), setup_tool)
            .add_systems(
                OnExit(PointerToolState::Active),
                despawn_recursive_with::<PointerTool>,
            )
            .add_systems(
                Update,
                (
                    debug_state_changes::<PointerToolState>,
                    handle_tool_change_events.run_if(on_event::<ToolChangedEvent>()),
                    (
                        // Run after `EditorCursorSet` to ensure we have the correct cursor status
                        handle_mouse_click
                            .after(EditorCursorSet)
                            .run_if(cursor_not_blocked),
                        handle_delete_keys,
                    )
                        .run_if(in_state(PointerToolState::Active)),
                ),
            );
    }
}

const TOOL_NAME: &str = "Pointer Tool";

/// State for the tool
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PointerToolState {
    #[default]
    Disabled,
    Active,
}

/// System that spawns the tool we enter the activated state.
fn setup_tool(mut commands: Commands) {
    info!("[{TOOL_NAME}] ==> setup");
    commands.spawn((
        Name::new(TOOL_NAME),
        PointerTool,
        PointerToolSelection(None),
    ));
}

/// Handle click events, try select an entity and mark them as selected.
#[allow(clippy::too_many_arguments)]
fn handle_mouse_click(
    mut query: Query<&mut PointerToolSelection, With<PointerTool>>,
    buttons: Res<Input<MouseButton>>,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    family_child_query: Query<&FamilyChild>,
    pickable_query: Query<(), With<Pickable>>,
    mut select_writer: EventWriter<SelectEvent>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for mut selection in query.iter_mut() {
            let result = pick_entity_from_raycast(
                &cursor_ray,
                &mut raycast,
                &pickable_query,
                &family_child_query,
            );
            let new_entity = result.map(|(entity, _)| entity);
            if new_entity != selection.0 {
                select_writer.send(SelectEvent(new_entity));
                selection.0 = new_entity;
            }
        }
    }
}

/// Systems which deletes the selected entity when the delete key is pressed.
fn handle_delete_keys(
    mut query: Query<&mut PointerToolSelection, With<PointerTool>>,
    keys: Res<Input<KeyCode>>,
    mut despawn_writer: EventWriter<DespawnEntityAndRelations>,
    not_deletable_query: Query<(), With<StaticLocked>>,
    mut select_writer: EventWriter<SelectEvent>,
) {
    if keys.just_pressed(KeyCode::Delete) {
        for mut selection in query.iter_mut() {
            if let Some(entity) = selection.0 {
                if not_deletable_query.contains(entity) {
                    warn!("Entity is not deletable");
                } else {
                    despawn_writer.send(DespawnEntityAndRelations(entity));
                    selection.0 = None;
                    select_writer.send(SelectEvent(None));
                }
            }
        }
    }
}

/// System that listens for `ToolChangedEvent` events and activates or de-activates the tool.
fn handle_tool_change_events(
    mut events: EventReader<ToolChangedEvent>,
    mut next_state: ResMut<NextState<PointerToolState>>,
    state: Res<State<PointerToolState>>,
) {
    for event in events.read() {
        if let Tool::Pointer = &event.0 {
            next_state.set(PointerToolState::Active);
        } else if *state.get() != PointerToolState::Disabled {
            next_state.set(PointerToolState::Disabled);
        }
    }
}
