use bevy::{prelude::*, reflect::GetTypeRegistration, utils::HashMap};

/// The global registry of types that should be tracked by `bevy_save`.
///
/// Only types that are registered in here and [`AppTypeRegistry`] are included in save/load and rollback.
#[derive(Resource, Default)]
pub struct SaveableRegistry {
    types: HashMap<String, bool>,
}

impl SaveableRegistry {
    /// Register a type to be included in saves and rollback.
    pub fn register<T: GetTypeRegistration>(&mut self) {
        let type_reg = T::get_type_registration();
        self.types
            .insert(type_reg.type_info().type_path().into(), true);
    }

    /// Returns whether a type name is registered in the [`SaveableRegistry`].
    pub fn contains(&self, type_name: &str) -> bool {
        self.types.contains_key(type_name)
    }

    /// Returns an iterator over registered type names.
    pub fn types(&self) -> impl Iterator<Item = &String> {
        self.types.keys()
    }
}
