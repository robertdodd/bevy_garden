use bevy::prelude::*;

/// The size of an apple mesh
pub(crate) const APPLE_MESH_SIZE: f32 = 1.0;

/// Resource holding the shared mesh and material used by apples
#[derive(Resource)]
pub(crate) struct AppleResource {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for AppleResource {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell.resource_mut::<Assets<StandardMaterial>>();
        let mut meshes = cell.resource_mut::<Assets<Mesh>>();

        Self {
            mesh: meshes.add(shape::Cube::new(APPLE_MESH_SIZE).into()),
            material: materials.add(Color::RED.into()),
        }
    }
}
