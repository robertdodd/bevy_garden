use bevy::prelude::*;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_recursive_with<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
    info!(
        "despawn_components({}) ==> despawned {} entities",
        std::any::type_name::<T>(),
        to_despawn.iter().len(),
    );
}

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn remove_components<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).remove::<T>();
    }
    info!(
        "remove_components({}) ==> removed from {} entities",
        std::any::type_name::<T>(),
        to_despawn.iter().len(),
    );
}

/// Generic system which initializes a resource implementing `FromWorld`
pub fn init_resource<T: Resource + FromWorld>(mut commands: Commands) {
    commands.init_resource::<T>();
}

/// Generic system which logs whenever a state changes
pub fn debug_state_changes<T: States>(state: Res<State<T>>) {
    if state.is_changed() && !state.is_added() {
        let result = std::any::type_name::<T>();
        let enum_type_name = match result.rsplit_once(':') {
            Some((_, s)) => s,
            None => result,
        };
        info!("[State Changed] ==> {}::{:?}", enum_type_name, state.get());
    }
}
