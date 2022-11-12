#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::too_many_arguments
)]
#![cfg(any(dim2, dim3))]

//! Physics behavior for Heron, using [rapier](https://rapier.rs/)
//!
//! # Supported custom collision shapes
//!
//! The following types are accepted as [`heron_core::CustomCollisionShape`]
//! values.

#[cfg(feature = "rapier2d")]
pub extern crate rapier2d;
#[cfg(feature = "rapier3d")]
pub extern crate rapier3d;

use bevy::prelude::*;
use bevy::time::TimePlugin;

pub use heron_rapier_v5::{
    convert, nalgebra, ColliderHandle, PhysicsWorld, RayCastInfo, RigidBodyHandle,
    ShapeCastCollisionInfo, ShapeCastCollisionType,
};

/// Plugin that enables collision detection and physics behavior, powered by rapier.
#[must_use]
#[derive(Debug, Copy, Clone, Default)]
pub struct RapierPlugin;

impl Plugin for RapierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TimePlugin)
            .add_plugin(heron_rapier_v5::RapierPlugin::default());
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::core::CorePlugin;

    use heron_core::{
        Acceleration, CollisionShape, PhysicsSteps, PhysicsTime, RigidBody, Velocity,
    };

    use super::*;

    #[test]
    fn updates_transform_before_transform_propagation() {
        for _ in 0..10 {
            let mut app = App::new();

            app.add_plugin(CorePlugin)
                .add_plugin(TransformPlugin)
                .add_plugin(RapierPlugin::default())
                .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)));

            let child = app
                .world
                .spawn()
                .insert_bundle((Transform::default(), GlobalTransform::default()))
                .id();

            app.world
                .spawn()
                .insert_bundle((
                    RigidBody::KinematicVelocityBased,
                    CollisionShape::Sphere { radius: 1.0 },
                    Velocity::from_linear(Vec3::X),
                    Transform::default(),
                    GlobalTransform::default(),
                ))
                .push_children(&[child]);

            app.update();

            assert_eq!(
                app.world
                    .get::<GlobalTransform>(child)
                    .unwrap()
                    .translation(),
                Vec3::X
            );
        }
    }

    #[test]
    fn does_not_update_rapier_when_paused() {
        let mut app = App::new();
        app.add_plugin(CorePlugin)
            .add_plugin(RapierPlugin::default())
            .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)));
        let entity = app
            .world
            .spawn()
            .insert_bundle((
                RigidBody::Dynamic,
                CollisionShape::Sphere { radius: 1.0 },
                Velocity::default(),
                Acceleration::from_linear(Vec3::X),
                Transform::default(),
                GlobalTransform::default(),
            ))
            .id();
        app.update();
        app.world.resource_mut::<PhysicsTime>().pause();
        app.update();
        app.world.resource_mut::<PhysicsTime>().resume();
        app.update();
        assert_eq!(app.world.get::<Velocity>(entity).unwrap().linear, Vec3::X);
    }
}
