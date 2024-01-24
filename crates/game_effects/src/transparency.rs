use bevy::prelude::*;

use bevy_helpers::material::clone_material_with_transparency;

const TRANSPARENT_MATERIAL_ALPHA: f32 = 0.75;

pub struct TransparentMaterialPlugin;

impl Plugin for TransparentMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_transparent_material, revert_transparent_material),
        );
    }
}

#[derive(Component)]
pub struct TransparentMaterial;

#[derive(Component)]
pub struct TransparentMaterialAlpha(pub f32);

#[derive(Component)]
struct TransparentMaterialConfig {
    initial: Handle<StandardMaterial>,
    disabled: Handle<StandardMaterial>,
}

/// Makes a disabled entity's material transparent
#[allow(clippy::type_complexity)]
fn update_transparent_material(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Handle<StandardMaterial>,
            Option<&TransparentMaterialConfig>,
            Option<&TransparentMaterialAlpha>,
        ),
        (
            With<TransparentMaterial>,
            Or<(
                Changed<Handle<StandardMaterial>>,
                Added<TransparentMaterial>,
            )>,
        ),
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut material_handle, transparent_material, alpha) in query.iter_mut() {
        warn!("[Transparency] ==> setting transparent material");
        if transparent_material
            .is_some_and(|transparent_material| *material_handle == transparent_material.disabled)
        {
            // material is up to date, ignore
            warn!("transparent material up to date, skipping");
            continue;
        }

        let alpha = alpha.map_or(TRANSPARENT_MATERIAL_ALPHA, |a| a.0);

        if let Some(material) = materials.get(material_handle.id()) {
            // create a new transparent version of the material
            let transparent_material = clone_material_with_transparency(material, alpha);
            let transparent_material_handle = materials.add(transparent_material);

            // insert reference to original material
            commands.entity(entity).insert(TransparentMaterialConfig {
                initial: material_handle.clone(),
                disabled: transparent_material_handle.clone(),
            });

            // replace material with the disabled one
            *material_handle = transparent_material_handle;
        }
    }
}

/// Reverts an entities material to it's original handle when the `Transparency` component is removed
fn revert_transparent_material(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Handle<StandardMaterial>,
            &TransparentMaterialConfig,
        ),
        Without<TransparentMaterial>,
    >,
) {
    for (entity, mut material, transparent_material) in query.iter_mut() {
        warn!("[Transparency] ==> reverting transparent material");
        *material = transparent_material.initial.clone();
        commands
            .entity(entity)
            .remove::<TransparentMaterialConfig>();
    }
}
