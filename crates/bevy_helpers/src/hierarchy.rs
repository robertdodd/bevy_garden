use bevy::prelude::{Children, Entity, Query};

pub fn recursively_get_children(entity: Entity, children_query: &Query<&Children>) -> Vec<Entity> {
    let mut result = Vec::<Entity>::new();
    result.push(entity);
    if let Ok(children) = children_query.get(entity) {
        result.extend(children);
        children_query.iter_many(children).for_each(|c| {
            result.extend(
                c.iter()
                    .flat_map(|e| recursively_get_children(*e, children_query)),
            );
        });
    }
    result
}
