#![allow(missing_docs)]

//! Type conversions between bevy and nalgebra (used by rapier's API)
//!
//! Names are 'bevy' centered.
//! * To convert from nalgebra to bevy use `from_xxx`
//! * To convert from bevy to nalgebra  use `to_xxx`

use crate::nalgebra;
use crate::rapier;
use bevy_math::prelude::*;

#[inline]
#[must_use]
#[cfg(feature = "3d")]
pub fn from_vector(v: nalgebra::Vector3<f32>) -> Vec3 {
    Vec3::new(v.x, v.y, v.z)
}

#[inline]
#[must_use]
#[cfg(feature = "2d")]
pub fn from_vector(v: nalgebra::Vector2<f32>) -> Vec3 {
    Vec3::new(v.x, v.y, 0.0)
}

#[inline]
#[must_use]
pub fn from_translation(translation: rapier::math::Translation<f32>) -> Vec3 {
    from_vector(translation.vector)
}

#[inline]
#[must_use]
#[cfg(feature = "3d")]
pub fn from_rotation(quaternion: nalgebra::UnitQuaternion<f32>) -> Quat {
    Quat::from_xyzw(quaternion.i, quaternion.j, quaternion.k, quaternion.w)
}

#[inline]
#[must_use]
#[cfg(feature = "2d")]
pub fn from_rotation(quaternion: nalgebra::UnitComplex<f32>) -> Quat {
    Quat::from_axis_angle(Vec3::unit_z(), -quaternion.angle())
}

#[inline]
#[must_use]
pub fn from_isometry(isometry: rapier::math::Isometry<f32>) -> (Vec3, Quat) {
    (
        from_translation(isometry.translation),
        from_rotation(isometry.rotation),
    )
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
