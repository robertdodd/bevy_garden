use bevy::prelude::*;

/// Resource holding the shared meshes and materials used by flowers
#[derive(Resource)]
pub(crate) struct FlowerResource {
    pub seed_mesh: Handle<Mesh>,
    pub petal_mesh: Handle<Mesh>,
    pub seed_material: Handle<StandardMaterial>,
    pub petal_material: Handle<StandardMaterial>,
    size: f32,
}

impl FlowerResource {
    pub fn size(&self) -> f32 {
        self.size
    }
}

impl FromWorld for FlowerResource {
    fn from_world(world: &mut World) -> Self {
        let size = 1.0;

        let cell = world.cell();
        let mut materials = cell.resource_mut::<Assets<StandardMaterial>>();
        let mut meshes = cell.resource_mut::<Assets<Mesh>>();

        Self {
            seed_mesh: meshes.add(shape::Cube::new(size).into()),
            petal_mesh: meshes.add(shape::Cube::new(size).into()),
            petal_material: materials.add(Color::YELLOW.into()),
            seed_material: materials.add(Color::BLACK.into()),
            size,
        }
    }
}
