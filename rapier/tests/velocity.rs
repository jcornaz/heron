#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::*;
use heron_rapier::convert::{IntoBevy, IntoRapier};
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
use heron_rapier::{BodyHandle, RapierPlugin};
use std::f32::consts::PI;

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
    let angular = AxisAngle::new(Vec3::unit_z(), 2.0);

    let entity = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Body::Sphere { radius: 1.0 },
        Velocity { linear, angular },
    ));

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();

    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

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

    let entity = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Body::Sphere { radius: 1.0 },
    ));

    app.update();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular = AxisAngle::new(Vec3::unit_z(), 2.0);

    app.world
        .insert_one(entity, Velocity { linear, angular })
        .unwrap();

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();

    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

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

    let entity = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Body::Sphere { radius: 1.0 },
        Velocity::default(),
    ));

    app.update();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular: AxisAngle = AxisAngle::new(Vec3::unit_z(), PI * 0.5);

    {
        let mut bodies = app.resources.get_mut::<RigidBodySet>().unwrap();
        let body = bodies
            .get_mut(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap();

        body.set_linvel(linear.into_rapier(), false);
        body.set_angvel(angular.into_rapier(), false);
    }

    app.update();

    let velocity = app.world.get::<Velocity>(entity).unwrap();

    assert_eq!(velocity.linear.x, linear.x);
    assert_eq!(velocity.linear.y, linear.y);

    #[cfg(feature = "3d")]
    assert_eq!(velocity.linear.z, linear.z);

    assert_eq!(angular, velocity.angular.into());
}

#[test]
fn velocity_can_move_kinematic_bodies() {
    let mut app = test_app();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular = AxisAngle::new(Vec3::unit_z(), PI * 0.5);

    let entity = app.world.spawn((
        GlobalTransform::from_rotation(Quat::from_axis_angle(Vec3::unit_z(), 0.0)),
        Body::Sphere { radius: 1.0 },
        BodyType::Kinematic,
        Velocity::from_linear(linear).with_angular(angular),
    ));

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();
    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    let position = body.position().translation.into_bevy();
    let rotation = body.position().rotation.into_bevy();

    assert_eq!(position.x, linear.x);
    assert_eq!(position.y, linear.y);

    #[cfg(feature = "3d")]
    assert_eq!(position.z, linear.z);

    let angular: Quat = angular.into();
    assert!((rotation.x - angular.x).abs() < 0.00001);
    assert!((rotation.y - angular.y).abs() < 0.00001);
    assert!((rotation.z - angular.z).abs() < 0.00001);
    assert!((rotation.w - angular.w).abs() < 0.00001);
}
