use bevy::{
    ecs::{archetype::Archetypes, component::Components, entity::Entities},
    prelude::*,
};

pub fn debug_entity(
    entity: Entity,
    name: Option<&Name>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    info!("Entity: {entity:?} ({name:?})");
    if let Some(entity_location) = entities.get(entity) {
        if let Some(archetype) = archetypes.get(entity_location.archetype_id) {
            for component in archetype.components() {
                if let Some(info) = components.get_info(component) {
                    info!("\tComponent: {}", info.name());
                }
            }
        }
    }
}

pub fn debug_entity_compact(
    entity: Entity,
    name: Option<&Name>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    info!("Entity: {entity:?} ({name:?})");
    if let Some(entity_location) = entities.get(entity) {
        if let Some(archetype) = archetypes.get(entity_location.archetype_id) {
            let component_names: Vec<String> = archetype
                .components()
                .filter_map(|component| components.get_info(component))
                .map(|info| info.name().to_string())
                .collect();
            info!("  {}", component_names.join(", "));
        }
    }
}

pub fn debug_all_entities(
    all_entities: &Query<(Entity, Option<&Name>)>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    for (entity, name) in all_entities.iter() {
        debug_entity(entity, name, entities, archetypes, components);
    }
}

pub fn debug_all_entities_compact(
    all_entities: &Query<(Entity, Option<&Name>)>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    for (entity, name) in all_entities.iter() {
        debug_entity_compact(entity, name, entities, archetypes, components);
    }
}

/// Utility that prints all entities and their name if they have one
pub fn debug_all_entity_ids(all_entities: &Query<(Entity, Option<&Name>)>) {
    for (entity, name) in all_entities.iter() {
        info!("Entity: {entity:?} {name:?}");
    }
}

/// Utility that prints the total number of entities
pub fn debug_entity_count(all_entities: &Query<(Entity, Option<&Name>)>) {
    info!("{} entities", all_entities.iter().count());
}
