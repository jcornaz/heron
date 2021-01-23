use bevy_math::prelude::*;

/// Component that defines the linear velocity in unit-per-second in every axis
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
///
/// fn spawn(commands: &mut Commands) {
///     commands.spawn(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .with(Body::Sphere { radius: 1.0 })
///         .with(LinearVelocity::from(Vec2::unit_x() * 10.0));
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Linear(Vec3);

/// Component that defines the angular velocity in radians-per-second around an axis
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// # use std::f32::consts::PI;
///
/// fn spawn(commands: &mut Commands) {
///     commands.spawn(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .with(Body::Sphere { radius: 1.0 })
///         .with(AngularVelocity::from_axis_angle(Vec3::unit_z(), 0.5 * PI));
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Angular(Quat);

impl From<Vec2> for Linear {
    fn from(v: Vec2) -> Self {
        Self::from(v.extend(0.0))
    }
}

impl From<Vec3> for Linear {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl From<Linear> for Vec3 {
    fn from(Linear(v): Linear) -> Self {
        v
    }
}

impl Angular {
    /// Create an angular velocity from a given angle (in radians) per second around the z axis
    #[must_use]
    pub fn from_angle(angle: f32) -> Self {
        Self::from_axis_angle(Vec3::unit_z(), angle)
    }

    /// Create an angular velocity from a given angle (in radians) per second around the given axis axis
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        Self::from(Quat::from_axis_angle(axis, angle))
    }
}

impl From<Quat> for Angular {
    fn from(quat: Quat) -> Self {
        Self(quat)
    }
}

impl From<Angular> for Quat {
    fn from(Angular(quat): Angular) -> Self {
        quat
    }
}
