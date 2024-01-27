use bevy::prelude::*;

use game_effects::transparency::*;
use game_state::prelude::*;
use save::prelude::*;

use crate::components::*;

/// Plugin which handles level backgrounds, which define the size, color and lighting for a level
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_new_backgrounds.in_set(SetupSet::RigidBody))
            .register_saveable::<Background>();
    }
}

fn setup_new_backgrounds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Background, Has<Disabled>), Added<Background>>,
) {
    for (entity, background, disabled) in query.iter() {
        info!("[Background] ==> Setup new background");

        // Set up the clear color resource
        commands.insert_resource(ClearColor(background.background_color));

        // Enable ambient lighting
        commands.insert_resource(AmbientLight {
            brightness: background.ambient_light,
            ..default()
        });

        let mut cmds = commands.entity(entity);

        // Add required components to the background
        cmds.insert((
            GameMarker,
            SpatialBundle::default(),
            AcceptsAttachables,
            StaticLocked,
        ));

        // Handle disabled marker
        if !disabled {
            cmds.insert((Saveable, DespawnOnLoad));
        }

        // Spawn directional lighting as a child of the Background
        cmds.with_children(|p| {
            p.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    shadows_enabled: true,
                    illuminance: background.directional_light,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO)
                    .looking_at(Vec3::new(-0.5, -1.0, -0.5), Vec3::Y),
                ..default()
            });
        });

        // Spawn the ground mesh as a child of the background
        cmds.with_children(|p| {
            let mut mesh_cmds = p.spawn((
                PbrBundle {
                    mesh: meshes.add(shape::Plane::from_size(background.size).into()),
                    material: materials.add(background.ground_color.into()),
                    ..default()
                },
                FamilyChild(entity),
            ));
            if disabled {
                mesh_cmds.insert(TransparentMaterial);
            } else {
                mesh_cmds.insert(Pickable);
            }
        });
    }
}
