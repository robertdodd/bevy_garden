use bevy::prelude::*;

use bevy_helpers::generic_systems::init_resource;
use game_effects::transparency::TransparentMaterial;
use game_state::prelude::*;
use save::prelude::*;

use crate::{components::*, resource::*};

pub struct FlowerPlugin;

impl Plugin for FlowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayState::SetupResources),
            init_resource::<FlowerResource>,
        )
        // Flowers should be setup in the `SetupSet::Attachable` so that their parent can be set up first.
        // NOTE: its not important in this example, but if you are using a physics engine then it's important for
        // RigidBodies/Joints to be spawned in the correct order.
        .add_systems(Update, setup_new_apples.in_set(SetupSet::Attachable))
        .register_saveable::<Flower>();
    }
}

#[allow(clippy::type_complexity)]
fn setup_new_apples(
    mut commands: Commands,
    query: Query<(Entity, &Transform, Has<Disabled>), Added<Flower>>,
    flower_resource: Res<FlowerResource>,
) {
    for (entity, transform, disabled) in query.iter() {
        info!("[Flower] ==> Setup new flower");

        // Set up the apple
        let mut cmds = commands.entity(entity);
        cmds.insert((
            Name::new("Flower"),
            GameMarker,
            Attachable,
            // We don't want to register all components from `SpatialBundle` in save files, so instead we only save
            // the transform and initialize a `SpatialBundle` with that transform each time its loaded.
            SpatialBundle::from_transform(*transform),
        ))
        .with_children(|p| {
            // spawn the seed mesh
            let mut seed_cmds = p.spawn((
                Name::new("Flower - Seed Mesh"),
                PbrBundle {
                    mesh: flower_resource.seed_mesh.clone(),
                    material: flower_resource.seed_material.clone(),
                    ..default()
                },
                FamilyChild(entity),
            ));
            if disabled {
                seed_cmds.insert(TransparentMaterial);
            } else {
                seed_cmds.insert(Pickable);
            }

            // spawn 4 petal meshes
            for pos in [
                Vec3::new(-flower_resource.size(), 0., 0.),
                Vec3::new(0., -flower_resource.size(), 0.),
                Vec3::new(flower_resource.size(), 0., 0.),
                Vec3::new(0., flower_resource.size(), 0.),
            ] {
                let mut petal_cmds = p.spawn((
                    Name::new("Flower - Petal Mesh"),
                    PbrBundle {
                        mesh: flower_resource.petal_mesh.clone(),
                        material: flower_resource.petal_material.clone(),
                        transform: Transform::from_translation(pos),
                        ..default()
                    },
                    FamilyChild(entity),
                ));
                if disabled {
                    petal_cmds.insert(TransparentMaterial);
                } else {
                    petal_cmds.insert(Pickable);
                }
            }
        });

        // only make it saveable when not disabled
        if !disabled {
            cmds.insert((Saveable, DespawnOnLoad));
        }
    }
}
