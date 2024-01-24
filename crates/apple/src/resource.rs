use bevy::prelude::*;

/// Resource holding the shared mesh and material used by all apples
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

        // The size of the apple mesh
        let size = 1.0;

        Self {
            mesh: meshes.add(shape::Cube::new(size).into()),
            material: materials.add(Color::RED.into()),
        }
    }
}
