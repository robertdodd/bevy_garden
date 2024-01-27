use bevy::prelude::*;

use bevy_helpers::generic_systems::init_resource;
use game_effects::transparency::TransparentMaterial;
use game_state::prelude::*;
use save::prelude::*;

use crate::{components::*, resource::*};

/// Plugin responsible for Bushes.
///
/// Bushes are "PhysicsBody" objects (stand-alone objects) that accept attachables as children.
///
/// For demonstration purposes, bushes are made up of 3 separate entities; the original bush entity and 2 meshes.
/// This allows us to test the `ExternalRelations` component, which treats all 3 entities as the same "object".
///
/// For example, clicking a mesh selects the entire object, and deleting it deletes all relations.
pub struct BushPlugin;

impl Plugin for BushPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PlayState::SetupResources),
            init_resource::<BushClusterResource>,
        )
        .add_systems(
            Update,
            (
                setup_new_bushes.in_set(SetupSet::RigidBody),
                update_mesh_transforms.run_if(in_game),
            ),
        )
        .register_saveable::<Bush>();
    }
}

/// System that initializes newly added `Bush` entities.
#[allow(clippy::type_complexity)]
fn setup_new_bushes(
    mut commands: Commands,
    query: Query<(Entity, &Transform, Has<Disabled>, Option<&Parent>), Added<Bush>>,
    bush_cluster_resource: Res<BushClusterResource>,
) {
    for (entity, transform, disabled, parent) in query.iter() {
        info!("[Bush] ==> Setup new bush");

        // spawn 2 bush meshes as separate entities.
        let half_size = BUSH_MESH_SIZE / 2.;
        let offset_x = half_size * 1.25;
        let bush_definitions = [
            Vec3::new(-offset_x, half_size, 0.),
            Vec3::new(offset_x, half_size, 0.),
        ];
        let bush_meshes: Vec<Entity> = bush_definitions
            .iter()
            .map(|local_anchor| {
                spawn_mesh(
                    &mut commands,
                    &bush_cluster_resource,
                    entity,
                    transform,
                    *local_anchor,
                    disabled,
                )
            })
            .collect();

        // If this entity has a parent, then parent the meshes to it.
        // NOTE: This should only happen if the bush mesh is a child of a `DynamicScene`, as is the case when placing
        // bushes using the prefab tool. This is important as it ensures all meshes will be despawned when that scene
        // despawns.
        if let Some(parent) = parent {
            for mesh_entity in bush_meshes.iter() {
                commands.entity(*mesh_entity).set_parent(parent.get());
            }
        }

        // Add required components to the bush.
        let mut cmds = commands.entity(entity);
        cmds.insert((
            Name::new("Bush"),
            GameMarker,
            SpatialBundle::from_transform(*transform),
            ExternalRelations(bush_meshes),
            PhysicsBody,
        ));

        // Handle the disabled marker.
        // If disabled, this entity should not be saved, nor should it have any components that allow it to be
        // interacted with.
        if !disabled {
            cmds.insert((Saveable, DespawnOnLoad, AcceptsAttachables));
        }
    }
}

/// Update the bush mesh transforms to follow the parent entity.
/// NOTE: This is only to demonstrate an object made up of separate entities, there would be no need for this if we
/// parented the meshes to the parent entity.
fn update_mesh_transforms(
    mut query: Query<(&mut Transform, &FamilyChild, &BushMesh)>,
    parent_query: Query<&Transform, (With<Bush>, Without<BushMesh>)>,
) {
    for (mut transform, family_child, mesh) in query.iter_mut() {
        if let Ok(parent_transform) = parent_query.get(family_child.0) {
            let mesh_transform = Transform::from_translation(mesh.local_anchor);
            *transform = parent_transform.mul_transform(mesh_transform);
        }
    }
}

/// Utility that spawns a bush mesh as a separate entity to test support for the custom `Family` hierarchy.
/// This is just an example. There is no need for the meshes to be separate entities outside the bevy hierarchy.
/// Usually, we'd only have separate entities with more complex meshes, or with an object made up of separate
/// colliders/joints when using a physics engine.
fn spawn_mesh(
    commands: &mut Commands,
    bush_cluster_resource: &BushClusterResource,
    parent_entity: Entity,
    parent_transform: &Transform,
    local_anchor: Vec3,
    disabled: bool,
) -> Entity {
    let mut cmds = commands.spawn((
        GameMarker,
        Name::new("Bush - Mesh"),
        BushMesh { local_anchor },
        PbrBundle {
            mesh: bush_cluster_resource.mesh.clone(),
            material: bush_cluster_resource.material.clone(),
            transform: parent_transform.mul_transform(Transform::from_translation(local_anchor)),
            ..default()
        },
        // Mark this mesh as a child belonging to the root `BushCluster` entity. This allows us to parent attachables to the
        // root entity instead of to this mesh, which is important because this mesh won't be included in save files.
        // Refer to the `prefab_tool` crate to see this in action when adding attachables.
        FamilyChild(parent_entity),
    ));

    // Handle disabled marker
    if disabled {
        cmds.insert(TransparentMaterial);
    } else {
        cmds.insert((Pickable, DespawnOnLoad));
    }

    cmds.id()
}
