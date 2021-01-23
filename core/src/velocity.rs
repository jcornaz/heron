use bevy_math::prelude::*;

/// Component that defines the linear and angular velocity.
///
/// The linear part is in "unit" per second on each axis, represented as a `Vec3`. (The unit, being your game unit, be it pixel or anything else)
/// The angular part is in radians per second around an axis, represented as a `Quat`
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
///         .with(
///             Velocity::from_linear(Vec3::unit_x() * 10.0)
///                 .with_angular(Quat::from_axis_angle(Vec3::unit_z(), 0.5 * PI))
///         );
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Velocity {
    /// Linear velocity in units-per-second on each axis
    pub linear: Vec3,

    /// Angular velocity in radians-per-second around an axis
    pub angular: Quat,
}

impl Velocity {
    /// Returns a linear velocity from a vector
    #[must_use]
    pub fn from_linear(linear: Vec3) -> Self {
        Self {
            linear,
            angular: Quat::identity(),
        }
    }

    /// Returns an angular velocity from a vector
    #[must_use]
    pub fn from_angular(angular: Quat) -> Self {
        Self {
            angular,
            linear: Vec3::zero(),
        }
    }

    /// Returns a new version with the given linear velocity
    #[must_use]
    pub fn with_linear(mut self, linear: Vec3) -> Self {
        self.linear = linear;
        self
    }

    /// Returns a new version with the given angular velocity
    #[must_use]
    pub fn with_angular(mut self, angular: Quat) -> Self {
        self.angular = angular;
        self
    }
}

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self::from_linear(v.extend(0.0))
    }
}

impl From<Vec3> for Velocity {
    fn from(linear: Vec3) -> Self {
        Self::from_linear(linear)
    }
}

impl From<Velocity> for Vec3 {
    fn from(Velocity { linear, .. }: Velocity) -> Self {
        linear
    }
}

impl From<Quat> for Velocity {
    fn from(quat: Quat) -> Self {
        Self::from_angular(quat)
    }
}

impl From<Velocity> for Quat {
    fn from(Velocity { angular, .. }: Velocity) -> Self {
        angular
    }
}
