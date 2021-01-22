#![allow(missing_docs)]

//! Type conversions between bevy and nalgebra (used by rapier's API)
//!
//! Provides the [`IntoBevy`](IntoBevy) and [`IntoRapier`](IntoRapier)
//! with implementations for bevy and rapier types

use bevy_math::prelude::*;

use crate::nalgebra::{self, UnitComplex, UnitQuaternion, Vector2, Vector3};
use crate::rapier;
use crate::rapier::math::{Isometry, Translation};

pub trait IntoBevy<T> {
    #[must_use]
    fn into_bevy(self) -> T;
}

pub trait IntoRapier<T> {
    #[must_use]
    fn into_rapier(self) -> T;
}

impl IntoBevy<Vec3> for Vector2<f32> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}

impl IntoBevy<Vec3> for Vector3<f32> {
    fn into_bevy(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl IntoBevy<Vec3> for Translation<f32> {
    fn into_bevy(self) -> Vec3 {
        self.vector.into_bevy()
    }
}

impl IntoBevy<Quat> for UnitComplex<f32> {
    fn into_bevy(self) -> Quat {
        Quat::from_axis_angle(Vec3::unit_z(), -self.angle())
    }
}

impl IntoBevy<Quat> for UnitQuaternion<f32> {
    fn into_bevy(self) -> Quat {
        Quat::from_xyzw(self.i, self.j, self.k, self.w)
    }
}

impl IntoBevy<(Vec3, Quat)> for Isometry<f32> {
    fn into_bevy(self) -> (Vec3, Quat) {
        (self.translation.into_bevy(), self.rotation.into_bevy())
    }
}

#[inline]
#[must_use]
#[cfg(feature = "3d")]
pub fn to_vector(v: Vec3) -> nalgebra::Vector3<f32> {
    nalgebra::Vector3::new(v.x, v.y, v.z)
}

#[inline]
#[must_use]
#[cfg(feature = "2d")]
pub fn to_vector(v: Vec3) -> nalgebra::Vector2<f32> {
    nalgebra::Vector2::new(v.x, v.y)
}

#[inline]
#[must_use]
pub fn to_translation(v: Vec3) -> rapier::math::Translation<f32> {
    rapier::math::Translation::from(to_vector(v))
}

#[inline]
#[must_use]
#[cfg(feature = "3d")]
pub fn to_rotation(rotation: Quat) -> nalgebra::UnitQuaternion<f32> {
    nalgebra::UnitQuaternion::new_normalize(nalgebra::Quaternion::new(
        rotation.w, rotation.x, rotation.y, rotation.z,
    ))
}

#[inline]
#[must_use]
#[cfg(feature = "2d")]
pub fn to_rotation(rotation: Quat) -> nalgebra::UnitComplex<f32> {
    let (_, angle) = rotation.to_axis_angle();
    nalgebra::UnitComplex::new(-angle)
}

#[inline]
#[must_use]
pub fn to_isometry(translation: Vec3, rotation: Quat) -> rapier::math::Isometry<f32> {
    rapier::math::Isometry::from_parts(to_translation(translation), to_rotation(rotation))
}

#[cfg(all(test, feature = "3d"))]
mod tests {
    use std::f32::consts::PI;

    use bevy_math::{Quat, Vec3};

    use super::*;

    mod isometry {
        use super::*;

        #[test]
        fn set_translation() {
            let translation = Vec3::new(1.0, 2.0, 3.0);
            let result = to_isometry(translation, Quat::identity());
            assert_eq!(translation.x, result.translation.x);
            assert_eq!(translation.y, result.translation.y);
            assert_eq!(translation.z, result.translation.z);
        }

        #[test]
        fn set_rotation() {
            let angle = PI / 2.0;
            let axis = Vec3::new(1.0, 2.0, 3.0);

            let quat = Quat::from_axis_angle(axis, angle).normalize();
            let result = to_isometry(Vec3::default(), quat);

            assert_eq!(result.rotation.i, quat.x);
            assert_eq!(result.rotation.j, quat.y);
            assert_eq!(result.rotation.k, quat.z);
            assert_eq!(result.rotation.w, quat.w);
        }
    }
}
