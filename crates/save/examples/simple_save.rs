use bevy::{
    ecs::{archetype::Archetypes, component::Components, entity::Entities},
    prelude::*,
};
use rand::Rng;

use save::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SavePlugin))
        .insert_resource(AmbientLight {
            brightness: 0.9,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                setup_new_shapes,
                handle_input_keys,
                handle_debug_keys,
                on_level_load,
            ),
        )
        .register_type::<Shape>()
        .register_saveable::<Shape>()
        // The following types are automatically registered as saveable:
        // .register_saveable::<Transform>()
        // .register_saveable::<Name>()
        // .register_saveable::<Parent>()
        // .register_saveable::<Children>()
        .run();
}

const FILENAME: &str = "level.scn.ron";
const RANDOM_POSITION_RANGE_XY: f32 = 5.0;
const RANDOM_POSITION_RANGE_Z: f32 = 10.0;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Shape {
    width: f32,
    height: f32,
    length: f32,
    color: Color,
}

#[derive(Bundle)]
struct ShapeBundle {
    name: Name,
    shape: Shape,
    spatial_bundle: SpatialBundle,
    saveable: Saveable,
    despawn_on_load: DespawnOnLoad,
}

impl ShapeBundle {
    /// Create a new shape with a random size, position and color.
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            name: Name::new("Shape"),
            shape: Shape {
                width: rng.gen_range(0.5..=2.0),
                height: rng.gen_range(0.5..=2.0),
                length: rng.gen_range(0.5..=2.0),
                color: Color::rgb(
                    rng.gen_range(0.0..=1.0),
                    rng.gen_range(0.0..=1.0),
                    rng.gen_range(0.0..=1.0),
                ),
            },
            spatial_bundle: SpatialBundle::from_transform(Transform::from_xyz(
                rng.gen_range(-RANDOM_POSITION_RANGE_XY..=RANDOM_POSITION_RANGE_XY),
                rng.gen_range(-RANDOM_POSITION_RANGE_XY..=RANDOM_POSITION_RANGE_XY),
                rng.gen_range(-RANDOM_POSITION_RANGE_Z..=RANDOM_POSITION_RANGE_Z),
            )),
            saveable: Saveable,
            despawn_on_load: DespawnOnLoad,
        }
    }
}

fn setup(mut commands: Commands, mut save_rollback_events: EventWriter<RollbackSaveEvent>) {
    // Obligatory camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Obligatory light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Create an initial shape
    commands.spawn(ShapeBundle::new_random());

    // Save initial rollback
    save_rollback_events.send(RollbackSaveEvent);
}

/// Setup newly spawned shapes
fn setup_new_shapes(
    mut commands: Commands,
    query: Query<(Entity, &Shape, &Transform), Added<Shape>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, shape_config, transform) in query.iter() {
        // initialize the mesh and any required components
        commands
            .entity(entity)
            .insert((
                // To keep our save file simple, we are only saving the `Transform`, `Name` and `Shape` components.
                // This means that whenever we load an entity we need to re-initiate the SpatialBundle, otherwise it
                // will be incomplete.
                // If you don't want to do this, you can register `GlobalTransform`, `Visibility` and
                // `ComputedVisibility` as saveable.
                // The same applies to the `DespawnOnLoad` and `Saveable` components, you're welcome to register them
                // as saveable.
                // However, doing it here keeps the save file small and easy to read. It also allows you to only add
                // components when needed, for example only adding `Saveable` when in editor mode etc.
                SpatialBundle::from_transform(*transform),
                DespawnOnLoad,
                Saveable,
            ))
            .with_children(|p| {
                p.spawn(PbrBundle {
                    mesh: meshes.add(
                        shape::Box::new(
                            shape_config.width,
                            shape_config.height,
                            shape_config.length,
                        )
                        .into(),
                    ),
                    material: materials.add(shape_config.color.into()),
                    ..default()
                });
            });
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_input_keys(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    query: Query<Entity, With<Shape>>,
    mut save_events: EventWriter<SaveEvent>,
    mut load_events: EventWriter<LoadEvent>,
    mut rollback_save_events: EventWriter<RollbackSaveEvent>,
    mut rollback_back_events: EventWriter<RollbackBackEvent>,
    mut rollback_forward_events: EventWriter<RollbackForwardEvent>,
) {
    // Delete a shape and save a rollback
    if keyboard.just_pressed(KeyCode::Back) {
        let first = query.iter().next();
        if let Some(entity) = first {
            commands.entity(entity).despawn_recursive();
            rollback_save_events.send(RollbackSaveEvent);
        }
    }

    // Create a new shape and save a rollback
    if keyboard.just_pressed(KeyCode::Space) {
        commands.spawn(ShapeBundle::new_random());
        rollback_save_events.send(RollbackSaveEvent);
    }

    // Save the level
    if keyboard.just_pressed(KeyCode::S) {
        save_events.send(SaveEvent {
            filename: FILENAME.to_string(),
            location: StorageLocation::Assets,
        });
    }

    // Load the level
    if keyboard.just_pressed(KeyCode::L) {
        load_events.send(LoadEvent {
            filename: FILENAME.to_string(),
            location: StorageLocation::Assets,
        });
    }

    // Previous rollback (Undo)
    if keyboard.just_pressed(KeyCode::B) {
        rollback_back_events.send(RollbackBackEvent);
    }

    // Next rollback (Redo)
    if keyboard.just_pressed(KeyCode::F) {
        rollback_forward_events.send(RollbackForwardEvent);
    }
}

/// Clear rollbacks when a level is loaded
fn on_level_load(
    mut events: EventReader<LevelLoadSuccess>,
    mut rollback_clear_events: EventWriter<RollbackClearEvent>,
) {
    for event in events.read() {
        info!("Successfully loaded {}", event.0);
        rollback_clear_events.send(RollbackClearEvent);
    }
}

/// Debug logging of all entities when the F1 key is pressed
fn handle_debug_keys(
    keyboard: Res<Input<KeyCode>>,
    all_entities: Query<(Entity, Option<&Name>)>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        for (entity, name) in all_entities.iter() {
            debug_entity(entity, name, entities, archetypes, components);
        }
    }
}

/// Prints an entity and all of it's components
fn debug_entity(
    entity: Entity,
    name: Option<&Name>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    info!("Entity: {entity:?} ({name:?})");
    if let Some(entity_location) = entities.get(entity) {
        if let Some(archetype) = archetypes.get(entity_location.archetype_id) {
            for component in archetype.components() {
                if let Some(info) = components.get_info(component) {
                    let name = info.name();
                    info!("\t{name}");
                }
            }
        }
    }
}
