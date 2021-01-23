#![allow(missing_docs)]

//! Type conversions between bevy and nalgebra (used by rapier's API)
//!
//! Provides the [`IntoBevy`](IntoBevy) and [`IntoRapier`](IntoRapier)
//! with implementations for bevy and rapier types

use bevy_math::prelude::*;

use crate::nalgebra::{self, Quaternion, UnitComplex, UnitQuaternion, Vector2, Vector3};
use crate::rapier::math::{Isometry, Translation, Vector};

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

impl IntoRapier<Vector2<f32>> for Vec2 {
    fn into_rapier(self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
}

impl IntoRapier<Vector2<f32>> for Vec3 {
    fn into_rapier(self) -> Vector2<f32> {
        self.truncate().into_rapier()
    }
}

impl IntoRapier<Vector3<f32>> for Vec3 {
    fn into_rapier(self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl IntoRapier<Translation<f32>> for Vec3 {
    fn into_rapier(self) -> Translation<f32> {
        <Vec3 as IntoRapier<Vector<f32>>>::into_rapier(self).into()
    }
}

impl IntoRapier<UnitComplex<f32>> for Quat {
    fn into_rapier(self) -> UnitComplex<f32> {
        let (_, angle) = self.to_axis_angle();
        nalgebra::UnitComplex::new(-angle)
    }
}

impl IntoRapier<UnitQuaternion<f32>> for Quat {
    fn into_rapier(self) -> UnitQuaternion<f32> {
        UnitQuaternion::new_normalize(Quaternion::new(self.w, self.x, self.y, self.z))
    }
}

impl IntoRapier<Isometry<f32>> for (Vec3, Quat) {
    fn into_rapier(self) -> Isometry<f32> {
        Isometry::from_parts(self.0.into_rapier(), self.1.into_rapier())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "3d")]
    use std::f32::consts::PI;

    use bevy_math::{Quat, Vec3};

    use super::*;

    mod into_isometry {
        use super::*;

        #[test]
        fn set_translation() {
            let translation = Vec3::new(1.0, 2.0, 3.0);
            let result = (translation, Quat::identity()).into_rapier();
            assert_eq!(translation.x, result.translation.x);
            assert_eq!(translation.y, result.translation.y);

            #[cfg(feature = "3d")]
            assert_eq!(translation.z, result.translation.z);
        }

        #[test]
        #[cfg(feature = "3d")]
        fn set_rotation() {
            let angle = PI / 2.0;
            let axis = Vec3::new(1.0, 2.0, 3.0);

            let quat = Quat::from_axis_angle(axis, angle).normalize();
            let result = (Vec3::default(), quat).into_rapier();

            assert_eq!(result.rotation.i, quat.x);
            assert_eq!(result.rotation.j, quat.y);
            assert_eq!(result.rotation.k, quat.z);
            assert_eq!(result.rotation.w, quat.w);
        }
    }
}
