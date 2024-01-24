use bevy::{asset::LoadState, input::mouse::MouseWheel, prelude::*};

use bevy_common_assets::ron::RonAssetPlugin;
use bevy_helpers::generic_systems::{debug_state_changes, despawn_recursive_with};
use bevy_scene_utils::commands::ExtractSceneToChildCommand;
use editor::prelude::*;
use game_state::prelude::*;
use save::prelude::RollbackSaveEvent;

use crate::{assets::*, attachable_cursor::*, commands::*, object_cursor::*, types::*};

/// Plugin which handles prefab tools defined by `ron` files
pub struct PrefabToolPlugin;

impl Plugin for PrefabToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PrefabToolResult>()
            .add_state::<PrefabToolState>()
            .configure_sets(Update, PrefabToolCursorSet.after(EditorCursorSet))
            .add_plugins((
                RonAssetPlugin::<PrefabTool>::new(&["tool.ron"]),
                ObjectCursorPlugin,
                AttachableCursorPlugin,
            ))
            .add_systems(OnEnter(PrefabToolState::Active), setup_tool)
            .add_systems(OnEnter(PrefabToolState::Reload), on_enter_reload)
            .add_systems(
                OnExit(PrefabToolState::Active),
                despawn_recursive_with::<OnPrefabTool>,
            )
            .add_systems(
                Update,
                (
                    debug_state_changes::<PrefabToolState>,
                    handle_tool_change_events.run_if(on_event::<ToolChangedEvent>()),
                    (
                        handle_place_attachable_events.run_if(on_event::<PlaceAttachableEvent>()),
                        handle_place_object_events.run_if(on_event::<PlaceObjectEvent>()),
                    )
                        .after(PrefabToolCursorSet),
                    handle_asset_loading.run_if(in_state(PrefabToolState::Active)),
                    handle_mouse_scroll.run_if(in_game),
                    handle_results.run_if(on_event::<PrefabToolResult>()),
                ),
            );
    }
}

const TOOL_NAME: &str = "Prefab Tool";
const SCALE_SPEED: f32 = 0.01;

/// State for the tool
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PrefabToolState {
    #[default]
    InActive,
    Active,
    Reload,
}

/// Transition from reload state to active state. This allows us to switch from one prefab tool to another.
fn on_enter_reload(mut next_state: ResMut<NextState<PrefabToolState>>) {
    next_state.set(PrefabToolState::Active);
}

fn setup_tool(
    mut commands: Commands,
    assets: Res<AssetServer>,
    path: Res<PrefabToolPath>,
    tool_assets: Res<Assets<PrefabTool>>,
) {
    info!("[{TOOL_NAME}] ==> setup");

    // Read the tool asset
    let handle = assets.load(path.0.clone());
    let tool = tool_assets.get(handle.id());
    if tool.is_none() {
        panic!("Could not load tool file: {}", path.0);
    }
    let tool = tool.unwrap();

    // Load the scene file
    let asset_path = handle.path().unwrap();
    let scene_path = asset_path.path().parent().unwrap().join("prefab.scn.ron");
    let scene_handle = assets.load(scene_path);

    // Spawn the tool cursor entity
    let tool_entity = commands
        .spawn((
            GameMarker,
            Name::new(format!("{TOOL_NAME} - {}", tool.name)),
            OnPrefabTool,
            PrefabToolCursor {
                name: tool.name.clone(),
                scene_handle: scene_handle.clone(),
                scaling: tool.scaling.clone(),
            },
            SpatialBundle::from_transform(
                Transform::IDENTITY.with_scale(Vec3::splat(tool.initial_scale)),
            ),
        ))
        .id();

    // Add the appropriate cursor component depending on how the tool needs to be placed
    match tool.tool_type {
        PrefabToolType::Attachable(ref config) => {
            commands.entity(tool_entity).insert(AttachableCursor {
                distance: config.distance,
                forward: config.forward,
            });
        }
        PrefabToolType::Object => {
            commands.entity(tool_entity).insert(ObjectCursor);
        }
    }
}

/// Handle click events for attachables
fn handle_place_attachable_events(
    mut commands: Commands,
    mut click_events: EventReader<PlaceAttachableEvent>,
    tool_query: Query<&PrefabToolCursor>,
) {
    for event in click_events.read() {
        if let Ok(prefab_tool) = tool_query.get(event.tool) {
            info!("[{TOOL_NAME} - {}] ==> Place attachable", prefab_tool.name);

            // Spawn the scene
            // NOTE: We use a custom command to spawn the scene because it allows us to operate on the scene before it
            // is spawned (updating positions, clearing velocities, change layers etc.).
            // Using `DynamicSceneBundle`s can cause issues because of their heirachies -- it's much cleaner if you
            // know that all top-level objects in the game do not have a parent.
            commands.add(SpawnPrefabCommand {
                tool_name: prefab_tool.name.clone(),
                scene_handle: prefab_tool.scene_handle.clone(),
                transform: event.transform,
                parent: Some(event.target),
            });
        }
    }
}

/// Handle click events for physics entities
#[allow(clippy::type_complexity)]
fn handle_place_object_events(
    mut commands: Commands,
    mut events: EventReader<PlaceObjectEvent>,
    tool_query: Query<&PrefabToolCursor>,
) {
    for event in events.read() {
        if let Ok(prefab_tool) = tool_query.get(event.tool) {
            info!("[{TOOL_NAME} - {}] ==> Place object", prefab_tool.name);

            // Spawn the scene
            // NOTE: We use a custom command to spawn the scene because it allows us to operate on the scene before it
            // is spawned (updating positions, clearing velocities, change layers etc.).
            // Using `DynamicSceneBundle`s can cause issues because of their heirachies -- it's much cleaner if you
            // know that all top-level objects in the game do not have a parent.
            commands.add(SpawnPrefabCommand {
                tool_name: prefab_tool.name.clone(),
                scene_handle: prefab_tool.scene_handle.clone(),
                transform: event.transform,
                parent: None,
            });
        }
    }
}

/// Wait for the tool's scene asset to finish loading, then spawn it as a child of the tool
#[allow(clippy::type_complexity)]
fn handle_asset_loading(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<(Entity, &PrefabToolCursor), Without<PrefabToolOk>>,
) {
    for (entity, prefab_tool) in query.iter() {
        if assets.get_load_state(prefab_tool.scene_handle.id()) == Some(LoadState::Loaded) {
            info!("[{TOOL_NAME}] => Scene loaded, spawning as child of tool");

            // Mark the tool as finished loading
            commands.entity(entity).insert(PrefabToolOk);

            // Spawn the scene as a child of the tool, mark it as disabled.
            // This custom command modifies the scene and then spawns it as a `DynamicSceneBundle`. In a more advanced
            // editor, you could use this command to extract any information about the scene that the tool should know
            // about, such as editable components, layer sizes etc.
            // TODO: Check that the scene contains a single entity first
            commands.add(ExtractSceneToChildCommand {
                scene: prefab_tool.scene_handle.clone(),
                entity,
                child_bundle: (Name::new("Scene Tool Scene"), OnPrefabTool, GameMarker),
                scene_bundle: (Disabled, GameMarker, OnPrefabTool),
            });
        }
    }
}

/// Handle activating and de-activating the tool
fn handle_tool_change_events(
    mut commands: Commands,
    mut events: EventReader<ToolChangedEvent>,
    mut next_state: ResMut<NextState<PrefabToolState>>,
    state: Res<State<PrefabToolState>>,
) {
    for event in events.read() {
        if let Tool::Prefab(path) = &event.0 {
            commands.insert_resource(PrefabToolPath(path.clone()));
            // handle changing to a different prefab while already activated
            if *state.get() == PrefabToolState::Active {
                next_state.set(PrefabToolState::Reload);
            } else {
                next_state.set(PrefabToolState::Active);
            }
        } else if *state.get() != PrefabToolState::InActive {
            next_state.set(PrefabToolState::InActive);
        }
    }
}

/// Scale the scene when the mouse is scrolled
fn handle_mouse_scroll(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &PrefabToolCursor)>,
) {
    for event in scroll_evr.read() {
        for (mut transform, cursor) in query.iter_mut() {
            if let Some(scaling) = cursor.scaling.as_ref() {
                let new_scale = (transform.scale.x + event.y * SCALE_SPEED)
                    .max(scaling.min)
                    .min(scaling.max);
                transform.scale = Vec3::splat(new_scale);
            }
        }
    }
}

fn handle_results(
    mut events: EventReader<PrefabToolResult>,
    mut rollback_writer: EventWriter<RollbackSaveEvent>,
) {
    for event in events.read() {
        match event.0.as_ref() {
            Ok(tool_name) => {
                info!("{} added successfully", tool_name);
                rollback_writer.send(RollbackSaveEvent);
            }
            Err(err) => error!("Prefab tool error: {err}"),
        }
    }
}
