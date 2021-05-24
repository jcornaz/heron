#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::app::ManualEventReader;
use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, RigidBody, SensorShape};
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    let mut parameters = IntegrationParameters::default();
    parameters.dt = 1.0;

    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin {
            step_per_second: None,
            parameters,
        })
        .add_system_to_stage(
            bevy::app::CoreStage::PostUpdate,
            bevy::transform::transform_propagate_system::transform_propagate_system.system(),
        );
    builder.app
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
        .get_resource::<ColliderSet>()
        .unwrap()
        .get(*app.world.get(entity).unwrap())
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
        .get_resource::<ColliderSet>()
        .unwrap()
        .get(*app.world.get(entity).unwrap())
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
        .get_resource::<ColliderSet>()
        .unwrap()
        .get(*app.world.get(entity).unwrap())
        .unwrap();

    assert!(!collider.is_sensor());
}
