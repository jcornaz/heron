#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, RigidBody};
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin {
            step_per_second: None,
            parameters: IntegrationParameters::default(),
        });
    builder.app
}

#[test]
fn create_dynamic_body() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            CollisionShape::Sphere { radius: 10.0 },
            RigidBody::Dynamic,
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert!(body.is_dynamic())
}

#[test]
fn create_static_body() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            CollisionShape::Sphere { radius: 10.0 },
            RigidBody::Static,
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert!(body.is_static())
}

#[test]
fn create_kinematic_body() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            CollisionShape::Sphere { radius: 10.0 },
            RigidBody::Kinematic,
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert!(body.is_kinematic())
}

#[test]
fn create_sensor_body() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            CollisionShape::Sphere { radius: 10.0 },
            RigidBody::Sensor,
        ))
        .id();

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let body = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert!(body.is_sensor())
}

#[test]
fn can_change_to_static_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
        ))
        .id();

    app.update();

    *app.world.entity_mut(entity).get_mut::<RigidBody>().unwrap() = RigidBody::Static;

    app.update();

    {
        let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
        let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

        assert!(body.is_static());
    }
}

#[test]
fn can_change_to_sensor_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
        ))
        .id();

    app.update();

    *app.world.entity_mut(entity).get_mut::<RigidBody>().unwrap() = RigidBody::Sensor;

    app.update();

    {
        let colliders = app.world.get_resource::<ColliderSet>().unwrap();
        let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

        assert!(collider.is_sensor());
    }
}

#[test]
fn can_change_to_dynamic_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            CollisionShape::Sphere { radius: 10.0 },
            RigidBody::Static,
        ))
        .id();

    app.update();

    *app.world.entity_mut(entity).get_mut::<RigidBody>().unwrap() = RigidBody::Dynamic;

    app.update();

    {
        let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
        let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

        assert!(body.is_dynamic());
    }
}
