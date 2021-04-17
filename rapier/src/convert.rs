#![allow(missing_docs)]

//! Type conversions between bevy and nalgebra (used by rapier's API)
//!
//! Provides the [`IntoBevy`](IntoBevy) and [`IntoRapier`](IntoRapier)
//! with implementations for bevy and rapier types

use bevy::math::prelude::*;

use heron_core::AxisAngle;

use crate::nalgebra::{
    self, Point2, Point3, Quaternion, UnitComplex, UnitQuaternion, Vector2, Vector3,
};
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
        Quat::from_axis_angle(Vec3::Z, self.angle())
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

impl IntoRapier<Point2<f32>> for Vec2 {
    fn into_rapier(self) -> Point2<f32> {
        Point2 {
            coords: self.into_rapier(),
        }
    }
}

impl IntoRapier<Point2<f32>> for Vec3 {
    fn into_rapier(self) -> Point2<f32> {
        Point2 {
            coords: self.into_rapier(),
        }
    }
}

impl IntoRapier<Point3<f32>> for Vec3 {
    fn into_rapier(self) -> Point3<f32> {
        Point3 {
            coords: self.into_rapier(),
        }
    }
}

impl IntoRapier<Vec<Point2<f32>>> for &[Vec3] {
    fn into_rapier(self) -> Vec<Point2<f32>> {
        self.iter().cloned().map(IntoRapier::into_rapier).collect()
    }
}

impl IntoRapier<Vec<Point3<f32>>> for &[Vec3] {
    fn into_rapier(self) -> Vec<Point3<f32>> {
        self.iter().cloned().map(IntoRapier::into_rapier).collect()
    }
}

impl IntoBevy<Vec2> for Point2<f32> {
    fn into_bevy(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl IntoBevy<Vec<Vec2>> for &[Point2<f32>] {
    fn into_bevy(self) -> Vec<Vec2> {
        self.iter().cloned().map(IntoBevy::into_bevy).collect()
    }
}

impl IntoRapier<Translation<f32>> for Vec3 {
    fn into_rapier(self) -> Translation<f32> {
        <Vec3 as IntoRapier<Vector<f32>>>::into_rapier(self).into()
    }
}

impl IntoRapier<UnitComplex<f32>> for Quat {
    fn into_rapier(self) -> UnitComplex<f32> {
        let (axis, angle) = self.to_axis_angle();
        nalgebra::UnitComplex::new(if axis.z > 0.0 { angle } else { -angle })
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

impl IntoRapier<f32> for AxisAngle {
    fn into_rapier(self) -> f32 {
        if self.axis().z > 0.0 {
            self.angle()
        } else {
            -self.angle()
        }
    }
}

impl IntoRapier<Vector3<f32>> for AxisAngle {
    fn into_rapier(self) -> Vector3<f32> {
        Vec3::from(self).into_rapier()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "3d")]
    use std::f32::consts::PI;

    use bevy::math::{Quat, Vec3};

    use super::*;

    #[cfg(feature = "2d")]
    mod angle2d {
        use rstest::rstest;

        use super::*;

        #[test]
        fn negative_axis_angle_to_rapier() {
            let result: f32 = AxisAngle::new(Vec3::Z, -1.0).into_rapier();
            assert_eq!(result, -1.0);
        }

        #[rstest(quat,
            case(Quat::from_axis_angle(Vec3::Z, 2.0)),
            case(Quat::from_axis_angle(-Vec3::Z, 2.0)),
            case(Quat::from_axis_angle(Vec3::Z, 0.0)),
        )]
        fn into_rapier_into_bevy_is_identity(quat: Quat) {
            let rapier: UnitComplex<f32> = quat.into_rapier();
            let actual: Quat = rapier.into_bevy();

            assert!((quat.x - actual.x).abs() < 0.0001);
            assert!((quat.y - actual.y).abs() < 0.0001);
            assert!((quat.z - actual.z).abs() < 0.0001);
            assert!((quat.w - actual.w).abs() < 0.0001);
        }
    }

    mod into_isometry {
        use super::*;

        #[test]
        fn set_translation() {
            let translation = Vec3::new(1.0, 2.0, 3.0);
            let result = (translation, Quat::IDENTITY).into_rapier();
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
