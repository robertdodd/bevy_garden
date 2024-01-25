//! This module contains common types used throughout the game.
//!
//! These types provide a standard ways for handling common game logic, and will be used by nearly all game-related
//! crates/modules.
use bevy::{prelude::*, reflect::Reflect};

/// Marker component for despawning all game entities when exiting the game state
#[derive(Component, Clone)]
pub struct GameMarker;

/// Marker component for the game camera
#[derive(Component)]
pub struct GameCamera;

/// Marker component for entities that are connected via a connector (i.e. a joint defined in another entity).
///
/// For example, if you have 2 `Shape` entities connected by a `Bolt` entity, all 3 entities should be given a
/// `HasConnector` component containing the other 2 entities that it is connected to. This allows the entire "object"
/// scene to be traversed.
///
/// Useful for capturing an entire object to a scene.
#[derive(Component)]
pub struct HasConnector(pub Vec<Entity>);

/// Marker component for entities that are rigid bodies in the physics engine.
///
/// We're not using physics in this example, but we still use this component to mark stand-alone objects.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PhysicsBody;

/// Marker component for objects that must be attached to a parent object.
///
/// These entities should always be added as a child via the bevy heirarchy.
#[derive(Component, Debug)]
pub struct Attachable;

/// Marker component added to disabled entities. When an object is initialized with this component present, it should
/// be initialized in a disabled state. This includes, but is not limited to, transparent mesh, disable picking,
/// disable saving, disabling physics etc.
///
/// It is the responsibility of each "game object" plugin to handle the presence of the `Disabled` component on its
/// entity.
///
/// It is not necessary to listen for the removal of this component -- one an entity is initialized with a `Disabled`
/// component, the `Disabled` component should not be removed.
#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Disabled;

/// Marker for entities that can be picked via raycasts
#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Pickable;

/// Marker for static entities that cannot be deleted or captured to scenes, such as the ground/background.
#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct StaticLocked;

/// Marker added to UI nodes that should block the cursor from interacting with the game world underneath.
///
/// It is used by the cursor tools, via the cursor status resource in `editor::editor_cursor`.
#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct PickableBlock;

/// Marker component for an object that allows attachables to be anchored to it
#[derive(Component, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct AcceptsAttachables;

/// Marks a component as a parent made up of a number of other entities that are not part of it's heirarchy.
///
/// This allows the external entities to be discovered, for example when adding wireframes, as well as to be deleted
/// when the parent is deleted.
#[derive(Component, Debug)]
pub struct ExternalRelations(pub Vec<Entity>);

/// Marks a component as a child of another entity. This is used to discover the parent entity when a child or
/// external mesh is selected.
#[derive(Component)]
pub struct FamilyChild(pub Entity);
