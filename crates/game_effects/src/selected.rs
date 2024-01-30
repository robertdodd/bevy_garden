use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
};

use game_state::prelude::*;

/// Plugin which adds wireframe rendering to selected entities
pub struct SelectedPlugin;

impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut App) {
        // Enable wireframes on desktop
        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(WireframePlugin)
            .add_systems(Startup, setup_wireframe_config);

        app.add_event::<SelectEvent>().add_systems(
            Update,
            (
                add_wireframe_to_selected_entities,
                remove_wireframe_from_de_selected_entities
                    .run_if(any_component_removed::<Selected>()),
                handle_select_events.run_if(on_event::<SelectEvent>()),
            )
                .run_if(in_game),
        );
    }
}

/// Marker component added to selected entities to add wireframe rendering
#[derive(Component)]
struct Selected;

/// Event used to select an entity and it's family
#[derive(Event)]
pub struct SelectEvent(pub Option<Entity>);

/// setup wireframe rendering in editor mode
#[cfg(not(target_arch = "wasm32"))]
fn setup_wireframe_config(mut wireframe_config: ResMut<WireframeConfig>) {
    // to render all wireframes, set to `true`. We only want to show wireframes for selected entities. When false, it
    // only renders wireframes for entities with a `Wireframe` component
    wireframe_config.global = false;
}

/// System that handles `SelectEvents` by adding `Selected` and `Wireframe` components to all entities in the target's
/// relations.
fn handle_select_events(
    mut commands: Commands,
    mut events: EventReader<SelectEvent>,
    family_child_query: Query<&FamilyChild>,
    external_relations_query: Query<&ExternalRelations>,
    selected_query: Query<Entity, With<Selected>>,
) {
    for event in events.read() {
        // Remove selected component from all other entities
        for entity in selected_query.iter() {
            commands.entity(entity).remove::<Selected>();
        }

        if let Some(entity) = event.0 {
            // Find the top-most entity if it's part of a family
            let parent = family_child_query
                .get(entity)
                .map_or(entity, |family_child| family_child.0);

            // Add selected component to top-most entity
            if let Some(mut cmds) = commands.get_entity(parent) {
                cmds.insert((Selected, Wireframe));
            }

            // Add selected component to external relations
            if let Ok(external_relations) = external_relations_query.get(parent) {
                for entity in external_relations.0.iter() {
                    if let Some(mut cmds) = commands.get_entity(*entity) {
                        cmds.insert((Selected, Wireframe));
                    }
                }
            }
        }
    }
}

/// System that adds `Wireframe` components to selected entities
#[allow(clippy::type_complexity)]
fn add_wireframe_to_selected_entities(
    mut commands: Commands,
    query: Query<(Entity, &Children, Has<Wireframe>), Added<Selected>>,
) {
    for (entity, children, has_wireframe) in query.iter() {
        // Add wireframe to all children
        for &child in children {
            if let Some(mut cmds) = commands.get_entity(child) {
                cmds.insert(Wireframe);
            }
        }

        // Add wireframe to this entity
        if !has_wireframe {
            if let Some(mut cmds) = commands.get_entity(entity) {
                cmds.insert(Wireframe);
            }
        }
    }
}

/// Remove the `Wireframe` marker from de-selected entities
fn remove_wireframe_from_de_selected_entities(
    mut commands: Commands,
    mut removed: RemovedComponents<Selected>,
    // entities with "removed" events may still have the "selected" component, if it was added again during the same
    // frame, so it's important to filter the query with `Without<Selected>`
    query: Query<(Option<&Children>, Has<Wireframe>), Without<Selected>>,
) {
    for entity in removed.read() {
        if let Ok((children, has_wireframe)) = query.get(entity) {
            // Remove wireframe from child entities
            if let Some(children) = children {
                for &child in children {
                    if let Some(mut cmds) = commands.get_entity(child) {
                        cmds.remove::<Wireframe>();
                    }
                }
            }

            // Remove wireframe from this entity
            if has_wireframe {
                commands.entity(entity).remove::<Wireframe>();
            }
        }
    }
}
