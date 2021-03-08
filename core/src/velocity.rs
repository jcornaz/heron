use bevy::math::prelude::*;
use bevy::reflect::prelude::*;

use crate::utils::NearZero;
use std::ops::{Mul, MulAssign};

/// Component that defines the linear and angular velocity.
///
/// The linear part is in "unit" per second on each axis, represented as a `Vec3`. (The unit, being your game unit, be it pixel or anything else)
/// The angular part is in radians per second around an axis, represented as a `Quat`.
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
///                 .with_angular(AxisAngle::new(Vec3::unit_z(), 0.5 * PI))
///         );
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Default, Reflect)]
pub struct Velocity {
    /// Linear velocity in units-per-second on each axis
    pub linear: Vec3,

    /// Angular velocity in radians-per-second around an axis
    pub angular: AxisAngle,
}

/// An [axis-angle] representation
///
/// [axis-angle]: https://en.wikipedia.org/wiki/Axis%E2%80%93angle_representation
#[derive(Debug, Copy, Clone, PartialEq, Default, Reflect)]
pub struct AxisAngle(Vec3);

impl Velocity {
    /// Returns a linear velocity from a vector
    #[must_use]
    pub fn from_linear(linear: Vec3) -> Self {
        Self {
            linear,
            angular: AxisAngle::default(),
        }
    }

    /// Returns an angular velocity from a vector
    #[must_use]
    pub fn from_angular(angular: AxisAngle) -> Self {
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
    pub fn with_angular(mut self, angular: AxisAngle) -> Self {
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

impl From<AxisAngle> for Velocity {
    fn from(angular: AxisAngle) -> Self {
        Self::from_angular(angular)
    }
}

impl From<Quat> for Velocity {
    fn from(quat: Quat) -> Self {
        Self::from_angular(quat.into())
    }
}

impl From<Velocity> for AxisAngle {
    fn from(Velocity { angular, .. }: Velocity) -> Self {
        angular
    }
}

impl From<Velocity> for Quat {
    fn from(Velocity { angular, .. }: Velocity) -> Self {
        angular.into()
    }
}

impl From<Vec3> for AxisAngle {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl From<AxisAngle> for Vec3 {
    fn from(AxisAngle(v): AxisAngle) -> Self {
        v
    }
}

impl NearZero for Velocity {
    fn is_near_zero(self) -> bool {
        self.linear.is_near_zero() && self.angular.is_near_zero()
    }
}

impl MulAssign<f32> for AxisAngle {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = self.0 * rhs;
    }
}

impl Mul<f32> for AxisAngle {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul<AxisAngle> for f32 {
    type Output = AxisAngle;

    fn mul(self, mut rhs: AxisAngle) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl AxisAngle {
    /// Create a new axis-angle
    #[inline]
    #[must_use]
    pub fn new(axis: Vec3, angle: f32) -> Self {
        Self(axis.normalize() * angle)
    }

    /// Squared angle.
    ///
    /// In general faster than `angle` because it doesn't need to perform a square-root
    #[inline]
    #[must_use]
    pub fn angle_squared(self) -> f32 {
        self.0.length_squared()
    }

    /// Angle around the axis.
    ///
    /// For comparison you may consider `angle_squared`, that doesn't need to perform a square root.
    #[inline]
    #[must_use]
    pub fn angle(self) -> f32 {
        self.0.length()
    }

    /// Returns the axis **NOT** normalized.
    #[inline]
    #[must_use]
    pub fn axis(self) -> Vec3 {
        self.0
    }
}

impl NearZero for AxisAngle {
    fn is_near_zero(self) -> bool {
        self.0.is_near_zero()
    }
}

impl From<Quat> for AxisAngle {
    fn from(quat: Quat) -> Self {
        let length = quat.length();
        let (axis, angle) = quat.to_axis_angle();
        Self(axis.normalize() * (angle * length))
    }
}

impl From<AxisAngle> for Quat {
    fn from(axis_angle: AxisAngle) -> Self {
        if axis_angle.is_near_zero() {
            Quat::identity()
        } else {
            let angle = axis_angle.0.length();
            Quat::from_axis_angle(axis_angle.0 / angle, angle)
        }
    }
}
