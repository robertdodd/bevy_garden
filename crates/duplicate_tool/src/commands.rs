use bevy::{ecs::system::Command, prelude::*};

use bevy_scene_utils::*;

use crate::types::CaptureObjectToSceneResult;

/// Command that extracts a scene from an entity and emits a `PickSceneResult` event when done.
///
/// The entire hierarchy of the entity is read and all resulting entities are captured to a scene, and all entities in
/// the scene are centered before saving it to the asset server.
///
/// The `PickSceneResult` event contains a `Result<Handle<DynamicScene>, String>`, which contains a handle to the
/// captured scene if successful, an error message otherwise.
#[derive(Debug)]
pub(crate) struct CaptureObjectToScene {
    pub entity: Entity,
}

impl Command for CaptureObjectToScene {
    fn apply(self, world: &mut World) {
        // create a scene from the entity
        let dynamic_scene = saveable_scene_from_entity(world, self.entity, true, true);
        let type_registry = world.resource::<AppTypeRegistry>();
        let scene = dynamic_scene_to_scene(&dynamic_scene, type_registry);

        if let Ok(mut scene) = scene {
            // center objects in the scene along the XZ axes.
            // NOTE: The XZ axes are specific to this app, where objects are placed on a fixed plane. If this were a
            // 2D platformer we might center them along the XY axes. If objects could be placed anywhere along the Y
            // axis, we would center them along all 3 axes.
            center_entities_in_scene(&mut scene, Vec3::new(1.0, 0.0, 1.0));

            // If using a physics engine, you should clear any `Velocity` components in the scene
            // set_velocity_in_scene(&mut scene, Vec2::ZERO, 0.0);

            // create a dynamic scene asset from the scene
            let mut assets = world
                .get_resource_mut::<Assets<DynamicScene>>()
                .expect("World does not have an Assets<DynamicScene> to add the new scene to");
            let dynamic_scene_handle = assets.add(DynamicScene::from_scene(&scene));

            world.send_event(CaptureObjectToSceneResult(Ok(dynamic_scene_handle)));
        } else if let Err(err) = scene {
            world.send_event(CaptureObjectToSceneResult(Err(err)));
        }
    }
}
