//! Type conversions between bevy and rapier

use bevy_math::*;

use crate::rapier::math::Vector;

#[inline]
#[cfg(feature = "2d")]
pub(crate) fn to_vector(v: Vec3) -> Vector<f32> {
    Vector::new(v.x, v.y)
}

#[inline]
#[cfg(feature = "3d")]
pub(crate) fn to_vector(v: Vec3) -> Vector<f32> {
    Vector::new(v.x, v.y, v.z)
}
