#![allow(missing_docs)]

//! Utility traits and extensions

use bevy::math::Vec3;

pub trait NearZero: Copy {
    fn is_near_zero(self) -> bool;
}

impl NearZero for f32 {
    fn is_near_zero(self) -> bool {
        self.abs() < f32::EPSILON
    }
}

impl NearZero for Vec3 {
    #[must_use]
    fn is_near_zero(self) -> bool {
        self.x.is_near_zero() && self.y.is_near_zero() && self.z.is_near_zero()
    }
}
