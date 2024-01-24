use bevy::prelude::*;

/// Resource holding the shared materials used by all trees
#[derive(Resource)]
pub(crate) struct TreeResource {
    pub trunk_material: Handle<StandardMaterial>,
    pub canopy_material: Handle<StandardMaterial>,
}

const TRUNK_COLOR: &str = "#784E3D";
const CANOPY_COLOR: &str = "#C4D7A4";

impl FromWorld for TreeResource {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell.resource_mut::<Assets<StandardMaterial>>();

        Self {
            trunk_material: materials.add(Color::hex(TRUNK_COLOR).unwrap().into()),
            canopy_material: materials.add(Color::hex(CANOPY_COLOR).unwrap().into()),
        }
    }
}
