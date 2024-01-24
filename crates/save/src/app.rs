use bevy::{prelude::*, reflect::GetTypeRegistration};

use super::registry::SaveableRegistry;

/// Extension trait that adds save-related methods to Bevy's [`App`].
pub trait AppSaveableExt {
    /// Register a type as saveable - it will be included in rollback and affected by save/load.
    fn register_saveable<T: GetTypeRegistration>(&mut self) -> &mut Self;
}

impl AppSaveableExt for App {
    fn register_saveable<T: GetTypeRegistration>(&mut self) -> &mut Self {
        self.init_resource::<SaveableRegistry>()
            .register_type::<T>();

        let mut registry = self.world.resource_mut::<SaveableRegistry>();
        registry.register::<T>();

        self
    }
}
