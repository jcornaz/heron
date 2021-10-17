use bevy::{ecs::component::Component, reflect::Reflect};

/// Component that restrict what rotations can be caused by forces.
///
/// It must be inserted on the same entity of a [`RigidBody`](crate::RigidBody)
///
/// Note that angular velocity may still be applied programmatically. This only restrict how rotation
/// can change when force/torques are applied.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
///
/// fn spawn(mut commands: Commands) {
///     commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .insert(CollisionShape::Sphere { radius: 1.0 })
///         .insert(RotationConstraints::lock()); // Prevent rotation caused by forces
/// }
/// ```
#[derive(Debug, Component, Copy, Clone, Reflect)]
pub struct RotationConstraints {
    /// Set to true to prevent rotations around the x axis
    pub allow_x: bool,

    /// Set to true to prevent rotations around the y axis
    pub allow_y: bool,

    /// Set to true to prevent rotations around the Z axis
    pub allow_z: bool,
}

impl Default for RotationConstraints {
    fn default() -> Self {
        Self::allow()
    }
}

impl RotationConstraints {
    /// Lock rotations around all axes
    #[must_use]
    pub fn lock() -> Self {
        Self {
            allow_x: false,
            allow_y: false,
            allow_z: false,
        }
    }

    /// Allow rotations around all axes
    #[must_use]
    pub fn allow() -> Self {
        Self {
            allow_x: true,
            allow_y: true,
            allow_z: true,
        }
    }

    /// Allow rotation around the x axis only (and prevent rotating around the other axes)
    #[must_use]
    pub fn restrict_to_x_only() -> Self {
        Self {
            allow_x: true,
            allow_y: false,
            allow_z: false,
        }
    }

    /// Allow rotation around the y axis only (and prevent rotating around the other axes)
    #[must_use]
    pub fn restrict_to_y_only() -> Self {
        Self {
            allow_x: false,
            allow_y: true,
            allow_z: false,
        }
    }

    /// Allow rotation around the z axis only (and prevent rotating around the other axes)
    #[must_use]
    pub fn restrict_to_z_only() -> Self {
        Self {
            allow_x: false,
            allow_y: false,
            allow_z: true,
        }
    }
}
