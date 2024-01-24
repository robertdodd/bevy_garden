use bevy::prelude::*;

/// System sets for setting up new entities in a specific order.
/// After loading a scene, the physics entities need to be set up first, followed by physics joints, and lastly any
/// "attachables"
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SetupSet {
    RigidBody,
    Joint,
    Attachable,
}
