use bevy::prelude::*;

use bevy_helpers::generic_systems::init_resource;
use game_effects::transparency::TransparentMaterial;
use game_state::prelude::*;
use save::prelude::*;

use crate::{components::*, resource::*};

/// Plugin which handles trees.
pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup the `FromWorld` resource in the appropriate state
            .add_systems(
                OnEnter(PlayState::SetupResources),
                init_resource::<TreeResource>,
            )
            // Trees should be setup in the `SetupSet::RigidBody` set because they are top-level objects, and need to be
            // set up before before any attachables that may be attached to them.
            // NOTE: its not important in this example, but if you are using a physics engine then its important for
            // RigidBodies/Joints to be spawned in the correct order.
            .add_systems(Update, setup_new_trees.in_set(SetupSet::RigidBody))
            // Register the types that should be included in save files.
            // NOTE: This automatically calls `register_type`.
            .register_saveable::<Tree>()
            .register_saveable::<TreeSize>();
    }
}

#[allow(clippy::type_complexity)]
fn setup_new_trees(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &TreeSize, &Transform, Has<Disabled>), Added<Tree>>,
    tree_resource: Res<TreeResource>,
) {
    for (entity, size, transform, disabled) in query.iter() {
        info!("[Tree] ==> Setup new tree");

        // Insert required components
        let mut cmds = commands.entity(entity);
        cmds.insert((
            Name::new("Tree"),
            GameMarker,
            PhysicsBody,
            // We don't want to register all components from `SpatialBundle` in save files, so instead we only save
            // the transform and initialize a `SpatialBundle` with that transform each time its loaded.
            SpatialBundle::from_transform(*transform),
        ));

        // Handle the disabled marker.
        // If disabled, this entity should not be saved, nor should it have any components that allow it to be
        // interacted with.
        if !disabled {
            cmds.insert((Saveable, DespawnOnLoad, AcceptsAttachables));
        }

        // spawn the trunk as a child of the tree
        spawn_trunk_mesh(
            &mut commands,
            &mut meshes,
            &tree_resource,
            entity,
            size,
            disabled,
        );

        // Spawn the canopy mesh as a child of the tree
        spawn_canopy_mesh(
            &mut commands,
            &mut meshes,
            &tree_resource,
            entity,
            size,
            disabled,
        );
    }
}

/// Spawn the trunk mesh as a child of a tree
fn spawn_trunk_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    tree_resource: &TreeResource,
    tree_entity: Entity,
    size: &TreeSize,
    disabled: bool,
) -> Entity {
    let mut cmds = commands.spawn((
        Name::new("Tree - Trunk Mesh"),
        PbrBundle {
            mesh: meshes.add(
                shape::Box::new(
                    size.trunk_thickness,
                    size.trunk_height,
                    size.trunk_thickness,
                )
                .into(),
            ),
            material: tree_resource.trunk_material.clone(),
            transform: Transform::from_xyz(0.0, size.trunk_height / 2.0, 0.),
            ..default()
        },
        // Mark this mesh as a child belonging to the root `Tree` entity. This allows us to parent attachables to the
        // root entity instead of to this mesh, which is important because this mesh won't be included in save files.
        // Refer to the `prefab_tool` crate to see this in action when adding attachables.
        FamilyChild(tree_entity),
    ));

    // Handle disabled marker
    // If not disabled, add the `Pickable` component to the mesh. If disabled, make the mesh transparent and don't
    // make it pickable.
    if disabled {
        cmds.insert(TransparentMaterial);
    } else {
        cmds.insert(Pickable);
    }

    let trunk = cmds.id();

    // Push as a child of the tree
    commands.entity(tree_entity).push_children(&[trunk]);

    trunk
}

/// Spawn the canopy mesh as a child of a tree
fn spawn_canopy_mesh(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    tree_resource: &TreeResource,
    tree_entity: Entity,
    size: &TreeSize,
    disabled: bool,
) -> Entity {
    let mut cmds = commands.spawn((
        Name::new("Tree - Canopy Mesh"),
        PbrBundle {
            mesh: meshes.add(shape::Box::new(size.canopy.x, size.canopy.y, size.canopy.z).into()),
            material: tree_resource.canopy_material.clone(),
            transform: Transform::from_xyz(0., size.trunk_height + size.canopy.y / 2., 0.),
            ..default()
        },
        // Mark this mesh as a child belonging to the root `Tree` entity. This allows us to parent attachables to the
        // root entity instead of to this mesh, which is important because this mesh won't be included in save files.
        // Refer to the `prefab_tool` crate to see this in action when adding attachables.
        FamilyChild(tree_entity),
    ));

    // Handle disabled marker
    // If not disabled, add the `Pickable` component to the mesh. If disabled, make the mesh transparent and don't
    // make it pickable.
    if disabled {
        cmds.insert(TransparentMaterial);
    } else {
        cmds.insert(Pickable);
    }

    let canopy = cmds.id();

    // Push as a child of the tree
    commands.entity(tree_entity).push_children(&[canopy]);

    canopy
}
