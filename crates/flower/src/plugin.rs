use bevy::prelude::*;

use bevy_helpers::generic_systems::init_resource;
use game_effects::transparency::TransparentMaterial;
use game_state::prelude::*;
use save::prelude::*;

use crate::{components::*, resource::*};

/// Plugin that adds flowers to the game.
///
/// Flowers are "attachable" objects, meaning they must be children of a `PhysicsBody` object.
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
        .add_systems(Update, setup_new_flowers.in_set(SetupSet::Attachable))
        .register_saveable::<Flower>();
    }
}

/// System that sets up newly added flowers.
fn setup_new_flowers(
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
            spawn_seed_mesh(entity, disabled, &flower_resource, p);
            for pos in [
                Vec3::new(-flower_resource.size(), 0., 0.),
                Vec3::new(0., -flower_resource.size(), 0.),
                Vec3::new(flower_resource.size(), 0., 0.),
                Vec3::new(0., flower_resource.size(), 0.),
            ] {
                spawn_petal_mesh(entity, disabled, pos, &flower_resource, p);
            }
        });

        // only make it saveable when not disabled
        if !disabled {
            cmds.insert((Saveable, DespawnOnLoad));
        }
    }
}

/// Utility that spawns a seed mesh for a flower.
fn spawn_seed_mesh(
    flower_entity: Entity,
    disabled: bool,
    flower_resource: &Res<FlowerResource>,
    parent: &mut ChildBuilder,
) {
    // spawn the seed mesh
    let mut seed_cmds = parent.spawn((
        Name::new("Flower - Seed Mesh"),
        PbrBundle {
            mesh: flower_resource.seed_mesh.clone(),
            material: flower_resource.seed_material.clone(),
            ..default()
        },
        FamilyChild(flower_entity),
    ));
    if disabled {
        seed_cmds.insert(TransparentMaterial);
    } else {
        seed_cmds.insert(Pickable);
    }
}

/// Utility that spawns a petal mesh for a flower.
fn spawn_petal_mesh(
    flower_entity: Entity,
    disabled: bool,
    translation: Vec3,
    flower_resource: &Res<FlowerResource>,
    parent: &mut ChildBuilder,
) {
    let mut petal_cmds = parent.spawn((
        Name::new("Flower - Petal Mesh"),
        PbrBundle {
            mesh: flower_resource.petal_mesh.clone(),
            material: flower_resource.petal_material.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
        FamilyChild(flower_entity),
    ));
    if disabled {
        petal_cmds.insert(TransparentMaterial);
    } else {
        petal_cmds.insert(Pickable);
    }
}
