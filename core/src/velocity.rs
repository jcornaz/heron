use bevy_math::prelude::*;

/// Linear velocity in unit/second in every axis
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Linear(Vec3);

/// Angular velocity
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
    /// Create an angular velocity from a given angle per second around the z axis
    #[must_use]
    pub fn from_angle(angle: f32) -> Self {
        Self::from_axis_angle(Vec3::unit_z(), angle)
    }

    /// Create an angular velocity from a given angle per second around the given axis axis
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
