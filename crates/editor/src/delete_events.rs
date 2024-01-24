use bevy::prelude::*;

use game_state::prelude::*;
use save::prelude::*;

/// Plugin which handles `DespawnEntityAndRelations` events, which despawn an entity and any related entities part of
/// it's "family" (i.e. related entities not in the bevy heirarchy).
pub struct DeleteEventPlugin;

impl Plugin for DeleteEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEntityAndRelations>().add_systems(
            Update,
            handle_delete_events.run_if(on_event::<DespawnEntityAndRelations>()),
        );
    }
}

/// An event used to delete an entire object and its relations, which may be separate entities.
#[derive(Event)]
pub struct DespawnEntityAndRelations(pub Entity);

/// System that reads `DespawnEntityAndRelations` events and deletes the entities.
fn handle_delete_events(
    mut commands: Commands,
    mut events: EventReader<DespawnEntityAndRelations>,
    family_child_query: Query<&FamilyChild>,
    relations_query: Query<&ExternalRelations>,
    mut rollback_writer: EventWriter<RollbackSaveEvent>,
) {
    for event in events.read() {
        // Find the top-most entity if the entity is part of a family
        let parent = family_child_query
            .get(event.0)
            .map_or(event.0, |family_child| family_child.0);

        // Despawn any external relations belonging to the entity
        if let Ok(external_relations) = relations_query.get(parent) {
            for entity in external_relations.0.iter() {
                if let Some(cmds) = commands.get_entity(*entity) {
                    cmds.despawn_recursive();
                }
            }
        }

        // despawn the entity
        if let Some(cmds) = commands.get_entity(parent) {
            cmds.despawn_recursive();
        }

        // Save a rollback
        rollback_writer.send(RollbackSaveEvent);
    }
}
