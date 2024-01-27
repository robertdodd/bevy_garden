use bevy::prelude::*;

use bevy_helpers::generic_systems::init_resource;
use game_effects::transparency::TransparentMaterial;
use game_state::prelude::*;
use save::prelude::*;

use crate::{components::*, resource::*};

/// Plugin which handles `Apple` entities
pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app
            // Set up the `FromWorld` resource in the appropriate state
            .add_systems(
                OnEnter(PlayState::SetupResources),
                init_resource::<AppleResource>,
            )
            // Apples should be setup in the `SetupSet::Attachable` so that their parent can be set up first.
            // NOTE: its not important in this example, but if you are using a physics engine then it's important for
            // RigidBodies/Joints to be spawned in the correct order.
            .add_systems(Update, setup_new_apples.in_set(SetupSet::Attachable))
            // Register the types that should be included in save files.
            // NOTE: This automatically calls `register_type`.
            .register_saveable::<Apple>();
    }
}

#[allow(clippy::type_complexity)]
fn setup_new_apples(
    mut commands: Commands,
    query: Query<(Entity, &Transform, Has<Disabled>), Added<Apple>>,
    apple_resource: Res<AppleResource>,
) {
    for (entity, transform, disabled) in query.iter() {
        info!("Setup new apple");

        // Set up the apple
        let mut cmds = commands.entity(entity);
        cmds.insert((
            Name::new("Apple"),
            GameMarker,
            Attachable,
            FamilyChild(entity),
            // We don't want to register all components from `SpatialBundle` in save files, so instead we only save
            // the transform and initialize a `SpatialBundle` with that transform each time its loaded.
            SpatialBundle::from_transform(*transform),
        ))
        .with_children(|p| {
            let mut mesh_cmds = p.spawn(PbrBundle {
                mesh: apple_resource.mesh.clone(),
                material: apple_resource.material.clone(),
                ..default()
            });
            if disabled {
                mesh_cmds.insert(TransparentMaterial);
            } else {
                mesh_cmds.insert(Pickable);
            }
        });

        // only make it saveable when not disabled
        if !disabled {
            cmds.insert((Saveable, DespawnOnLoad));
        }
    }
}
