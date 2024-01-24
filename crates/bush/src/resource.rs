use bevy::prelude::*;

/// Resource holding the shared materials used by all trees
#[derive(Resource)]
pub(crate) struct BushClusterResource {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    bush_size: f32,
}

impl BushClusterResource {
    pub fn size(&self) -> f32 {
        self.bush_size
    }
}

const BUSH_COLOR: &str = "#375D3B";

impl FromWorld for BushClusterResource {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell.resource_mut::<Assets<StandardMaterial>>();
        let mut meshes = cell.resource_mut::<Assets<Mesh>>();

        let bush_size = 1.;

        Self {
            mesh: meshes.add(shape::Cube::new(bush_size).into()),
            material: materials.add(Color::hex(BUSH_COLOR).unwrap().into()),
            bush_size,
        }
    }
}
