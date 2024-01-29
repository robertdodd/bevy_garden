use bevy::prelude::*;

use crate::{rollbacks::Rollbacks, types::*, utils::get_saveable_scene_filter_from_world};

use super::WriteSceneToWorldCommand;

// Despawn all entities with `Saveable` or `DespawnOnLoad` components.
#[allow(clippy::type_complexity)]
pub(crate) fn despawn_saveable_entities(
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Applies a scene handle to the world
#[allow(clippy::type_complexity)]
pub(crate) fn apply_scene_handle(
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    scene_handle: Handle<DynamicScene>,
) {
    despawn_saveable_entities(commands, query);
    let cmd = WriteSceneToWorldCommand {
        scene_handle: scene_handle.clone(),
    };
    commands.add(cmd);
}

/// Applies a rollback in the specified direction
#[allow(clippy::type_complexity)]
pub(crate) fn apply_rollback(
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    rollbacks: &mut ResMut<Rollbacks>,
    direction: isize,
) -> Result<(), ()> {
    if !rollbacks.has_checkpoint(direction) {
        return Err(());
    }
    if let Some(scene_handle) = rollbacks.rollback(direction) {
        apply_scene_handle(commands, query, scene_handle);
        Ok(())
    } else {
        Err(())
    }
}

/// Extracts a `DynamicScene` from the world.
pub(crate) fn saveable_scene_from_world(world: &mut World) -> DynamicScene {
    // extract the entities we want to save
    let mut query = world.query_filtered::<Entity, With<Saveable>>();
    let entities: Vec<Entity> = query.iter(world).collect();

    // get a scene filter from the world that only includes types registered in the `SaveableRegistry`
    let filter = get_saveable_scene_filter_from_world(world);

    // build the scene
    DynamicSceneBuilder::from_world(world)
        .with_filter(filter)
        .extract_entities(entities.into_iter())
        .remove_empty_entities()
        .build()
}
