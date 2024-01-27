use bevy::prelude::*;

/// the current level being loaded
#[derive(Resource)]
pub(crate) struct PendingLevelLoad {
    pub(crate) path: String,
    pub(crate) handle: Handle<DynamicScene>,
}

/// marker component for saveable entities
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Saveable;

/// marker component for entities that should be de-spawned on load
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DespawnOnLoad;

/// Location from which to save/load levels.
/// Game assets should be saved/loaded from the `Assets` location, while end-user content should be saved/loaded from
/// the `Workspace` location.
#[derive(Clone, Copy)]
pub enum StorageLocation {
    Assets,
    Workspace,
}
