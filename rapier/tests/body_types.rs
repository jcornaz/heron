#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{Body, BodyType};
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::{BodyHandle, RapierPlugin};

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

    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        BodyType::Dynamic,
    ));

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();
    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    assert!(body.is_dynamic())
}

#[test]
fn create_static_body() {
    let mut app = test_app();

    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        BodyType::Static,
    ));

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();
    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    assert!(body.is_static())
}

#[test]
fn create_sensor_body() {
    let mut app = test_app();

    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        BodyType::Sensor,
    ));

    app.update();

    let colliders = app.resources.get::<ColliderSet>().unwrap();
    let body = colliders
        .get(app.world.get::<BodyHandle>(entity).unwrap().collider())
        .unwrap();

    assert!(body.is_sensor())
}

#[test]
fn can_change_to_static_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn((GlobalTransform::default(), Body::Sphere { radius: 10.0 }));

    app.update();

    app.world.insert_one(entity, BodyType::Static).unwrap();

    app.update();

    {
        let bodies = app.resources.get::<RigidBodySet>().unwrap();
        let body = bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap();

        assert!(body.is_static());
    }
}

#[test]
fn can_change_to_sensor_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn((GlobalTransform::default(), Body::Sphere { radius: 10.0 }));

    app.update();

    app.world.insert_one(entity, BodyType::Sensor).unwrap();

    app.update();

    {
        let colliders = app.resources.get::<ColliderSet>().unwrap();
        let collider = colliders
            .get(app.world.get::<BodyHandle>(entity).unwrap().collider())
            .unwrap();

        assert!(collider.is_sensor());
    }
}

#[test]
fn can_change_to_dynamic_after_creation() {
    let mut app = test_app();

    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        BodyType::Static,
    ));

    app.update();

    app.world.insert_one(entity, BodyType::Dynamic).unwrap();

    app.update();

    {
        let bodies = app.resources.get::<RigidBodySet>().unwrap();
        let body = bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap();

        assert!(body.is_dynamic());
    }
}

#[test]
fn can_change_to_dynamic_by_removing_type_after_creation() {
    let mut app = test_app();

    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        BodyType::Static,
    ));

    app.update();

    app.world.remove_one::<BodyType>(entity).unwrap();

    app.update();

    {
        let bodies = app.resources.get::<RigidBodySet>().unwrap();
        let body = bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap();

        assert!(body.is_dynamic());
    }
}
