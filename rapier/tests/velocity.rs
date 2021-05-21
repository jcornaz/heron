#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use std::f32::consts::PI;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;
use rstest::rstest;

use heron_core::*;
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin {
            step_per_second: None,
            parameters: {
                let mut params = IntegrationParameters::default();
                params.dt = 1.0;
                params
            },
        });
    builder.app
}

#[test]
fn body_is_created_with_velocity() {
    let mut app = test_app();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular = AxisAngle::new(Vec3::Z, 2.0);

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
            Velocity { linear, angular },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();

    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    let actual_linear = (*body.linvel()).into_bevy();

    assert_eq!(linear.x, actual_linear.x);
    assert_eq!(linear.y, actual_linear.y);

    #[cfg(feature = "3d")]
    assert_eq!(linear.z, actual_linear.z);

    #[cfg(feature = "3d")]
    assert_eq!(angular, (*body.angvel()).into_bevy().into());

    #[cfg(feature = "2d")]
    assert_eq!(angular.angle(), body.angvel());
}

#[test]
fn velocity_may_be_added_after_creating_the_body() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
        ))
        .id();

    app.update();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular = AxisAngle::new(Vec3::Z, 2.0);

    app.world
        .entity_mut(entity)
        .insert(Velocity { linear, angular });

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();

    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    let actual_linear = (*body.linvel()).into_bevy();
    assert_eq!(linear.x, actual_linear.x);
    assert_eq!(linear.y, actual_linear.y);

    #[cfg(feature = "3d")]
    assert_eq!(linear.z, actual_linear.z);

    #[cfg(feature = "2d")]
    assert_eq!(angular.angle(), body.angvel());

    #[cfg(feature = "3d")]
    assert_eq!(angular, (*body.angvel()).into_bevy().into());
}

#[test]
fn velocity_is_updated_to_reflect_rapier_world() {
    let mut app = test_app();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular: AxisAngle = AxisAngle::new(Vec3::Z, PI * 0.5);

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
            Velocity::default(),
            Acceleration::from_linear(linear).with_angular(angular),
        ))
        .id();

    app.update();
    app.update();

    let velocity = app.world.get::<Velocity>(entity).unwrap();

    assert_eq!(velocity.linear.x, linear.x);
    assert_eq!(velocity.linear.y, linear.y);

    #[cfg(feature = "3d")]
    assert_eq!(velocity.linear.z, linear.z);

    #[cfg(feature = "3d")]
    assert_eq!(angular, velocity.angular.into());

    #[cfg(feature = "2d")]
    assert!((angular.angle() - velocity.angular.angle()).abs() < 0.001);
}

#[rstest]
#[case(Some(RigidBody::Dynamic))]
#[case(Some(RigidBody::Kinematic))]
#[case(None)]
fn velocity_can_move_kinematic_bodies(#[case] body_type: Option<RigidBody>) {
    let mut app = test_app();
    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::from_axis_angle(Vec3::Z, PI / 2.0);

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 2.0 },
            Transform::default(),
            GlobalTransform::default(),
            Velocity::from(translation).with_angular(rotation.into()),
        ))
        .id();

    if let Some(body_type) = body_type {
        app.world.entity_mut(entity).insert(body_type);
    }

    app.update();

    let Transform {
        translation: actual_translation,
        rotation: actual_rotation,
        ..
    } = *app.world.get::<Transform>(entity).unwrap();

    #[cfg(feature = "3d")]
    assert_eq!(actual_translation, translation);

    #[cfg(feature = "2d")]
    assert_eq!(actual_translation.truncate(), translation.truncate());

    let (axis, angle) = rotation.to_axis_angle();
    let (actual_axis, actual_angle) = actual_rotation.to_axis_angle();

    assert!(actual_axis.angle_between(axis) < 0.001);
    assert!((actual_angle - angle).abs() < 0.001);
}
