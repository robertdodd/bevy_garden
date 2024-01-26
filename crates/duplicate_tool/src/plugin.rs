use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use bevy_helpers::generic_systems::{debug_state_changes, despawn_recursive_with};
use editor::prelude::*;
use game_state::prelude::*;
use save::prelude::*;

use crate::{commands::CaptureObjectToScene, types::*};

/// Plugin which handles the "Duplicate" tool.
///
/// The duplicate tool lets you select an object and then place that object multiple times in the world.
///
/// It works by extracting the object to a scene (it extracts the entire object heirarchy) and transitions to the
/// "Prefab" tool to place the object.
pub struct DuplicateToolPlugin;

impl Plugin for DuplicateToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<DuplicateToolState>()
            .add_event::<CaptureObjectToSceneResult>()
            .add_systems(OnEnter(DuplicateToolState::Active), setup_tool)
            .add_systems(
                OnExit(DuplicateToolState::Active),
                despawn_recursive_with::<DuplicateTool>,
            )
            .add_systems(
                Update,
                (
                    debug_state_changes::<DuplicateToolState>,
                    handle_tool_change_events.run_if(on_event::<ToolChangedEvent>()),
                    handle_pick_scene_result_events
                        .run_if(on_event::<CaptureObjectToSceneResult>()),
                    handle_mouse_click
                        .in_set(EditorCursorSet::Click)
                        .run_if(cursor_not_blocked)
                        .run_if(in_state(DuplicateToolState::Active)),
                ),
            );
    }
}

const TOOL_NAME: &str = "Duplicate Tool";

/// State for the tool
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum DuplicateToolState {
    #[default]
    Disabled,
    Active,
}

/// System that spawns the tool we enter the activated state.
fn setup_tool(mut commands: Commands) {
    info!("[{TOOL_NAME}] ==> setup");
    commands.spawn((
        Name::new(TOOL_NAME),
        DuplicateTool,
        SpatialBundle::default(),
    ));
}

/// Handle click events, try select an entity and mark them as selected.
#[allow(clippy::too_many_arguments)]
fn handle_mouse_click(
    mut commands: Commands,
    mut raycast: Raycast,
    buttons: Res<Input<MouseButton>>,
    cursor_ray: Res<CursorRay>,
    query: Query<Entity, With<DuplicateTool>>,
    family_child_query: Query<&FamilyChild>,
    pickable_query: Query<(), With<Pickable>>,
    valid_target_query: Query<&Transform, (With<Saveable>, Without<StaticLocked>)>,
) {
    // STYLE: this reduces the nesting below
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    for _ in query.iter() {
        // pick an entity from the raycast, and make sure it matches `valid_target_query` and does not contains a
        // `StaticLocked` component.
        let target_entity = pick_entity_from_raycast(
            &cursor_ray,
            &mut raycast,
            &pickable_query,
            &family_child_query,
        )
        .map(|(entity, _)| entity)
        .filter(|entity| valid_target_query.contains(*entity));

        // If the raycast was successful, then send a command to extract it to a scene. The command will emit a
        // `CaptureObjectToSceneResult` event which we must handle in this plugin.
        if let Some(target_entity) = target_entity {
            commands.add(CaptureObjectToScene {
                entity: target_entity,
            });
        } else {
            warn!("Failed to select an object");
        }
    }
}

fn handle_pick_scene_result_events(
    mut events: EventReader<CaptureObjectToSceneResult>,
    state: Res<State<DuplicateToolState>>,
    query: Query<Entity, With<DuplicateTool>>,
    mut replace_tool_events: EventWriter<ReplaceToolEvent>,
) {
    // Handle the rare case when the tool is no longer active when we receive the result.
    if *state != DuplicateToolState::Active || query.is_empty() {
        events.clear();
        return;
    }

    for event in events.read() {
        match event.0.as_ref() {
            Ok(scene_handle) => {
                replace_tool_events.send(ReplaceToolEvent(Tool::Prefab(PrefabConfig {
                    name: "Object".to_string(),
                    tool_type: PrefabToolType::Object,
                    scene: scene_handle.clone(),
                    initial_scale: 1.,
                    scaling: Some(ToolScaling { min: 0.5, max: 2.0 }),
                })));
            }
            Err(err) => error!("[{TOOL_NAME}] ==> Error capturing object to scene: {err}"),
        }
    }
}

/// System that listens for `ToolChangedEvent` events and activates or de-activates the tool.
fn handle_tool_change_events(
    mut events: EventReader<ToolChangedEvent>,
    mut next_state: ResMut<NextState<DuplicateToolState>>,
    state: Res<State<DuplicateToolState>>,
) {
    for event in events.read() {
        if let Tool::Duplicate = &event.0 {
            next_state.set(DuplicateToolState::Active);
        } else if *state.get() != DuplicateToolState::Disabled {
            next_state.set(DuplicateToolState::Disabled);
        }
    }
}
