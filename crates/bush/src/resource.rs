use bevy::prelude::*;

pub(crate) const BUSH_COLOR: &str = "#375D3B";
pub(crate) const BUSH_MESH_SIZE: f32 = 1.0;

/// Resource holding the shared materials for bushes.
#[derive(Resource)]
pub(crate) struct BushClusterResource {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for BushClusterResource {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell.resource_mut::<Assets<StandardMaterial>>();
        let mut meshes = cell.resource_mut::<Assets<Mesh>>();

        Self {
            mesh: meshes.add(shape::Cube::new(BUSH_MESH_SIZE).into()),
            material: materials.add(Color::hex(BUSH_COLOR).unwrap().into()),
        }
    }
}
