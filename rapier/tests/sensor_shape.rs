#![cfg(any(dim2, dim3))]

use std::time::Duration;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, PhysicsSteps, RigidBody, SensorShape};
use heron_rapier::convert::IntoRapier;
use heron_rapier::{ColliderHandle, RapierPlugin};
use utils::*;

mod utils;

fn test_app() -> App {
    let mut builder = App::new();

    builder
        .init_resource::<TypeRegistryArc>()
        .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)))
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin);
    builder
}

#[test]
fn a_non_sensor_body_can_have_a_sensor_shape() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
            SensorShape,
        ))
        .id();

    app.update();

    let collider = app
        .world
        .resource::<ColliderSet>()
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();

    assert!(collider.is_sensor());
}

#[test]
fn sensor_flag_can_be_added_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).insert(SensorShape);

    app.update();

    let collider = app
        .world
        .resource::<ColliderSet>()
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();

    assert!(collider.is_sensor());
}

#[test]
fn sensor_flag_can_removed() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
            SensorShape,
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<SensorShape>();

    app.update();

    let collider = app
        .world
        .resource::<ColliderSet>()
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();

    assert!(!collider.is_sensor());
}

#[test]
fn removing_sensor_flag_has_no_effect_if_body_is_sensor() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Sensor,
            CollisionShape::Sphere { radius: 1.0 },
            SensorShape,
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<SensorShape>();

    app.update();

    let collider = app
        .world
        .resource::<ColliderSet>()
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();

    assert!(collider.is_sensor());
}
