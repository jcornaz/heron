#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use std::f32::consts::PI;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::*;
use heron_rapier::convert::{IntoBevy, IntoRapier};
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
use heron_rapier::{BodyHandle, RapierPlugin};

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
            Body::Sphere { radius: 1.0 },
            Velocity { linear, angular },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();

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

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            Body::Sphere { radius: 1.0 },
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
#[ignore]
fn velocity_is_updated_to_reflect_rapier_world() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            Body::Sphere { radius: 1.0 },
            Velocity::default(),
        ))
        .id();

    app.update();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular: AxisAngle = AxisAngle::new(Vec3::Z, PI * 0.5);

    {
        let rigid_body_handle = app.world.get::<BodyHandle>(entity).unwrap().rigid_body();
        let mut bodies = app.world.get_resource_mut::<RigidBodySet>().unwrap();
        let body = bodies.get_mut(rigid_body_handle).unwrap();

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
#[ignore]
fn velocity_can_move_kinematic_bodies() {
    let mut app = test_app();

    let linear = Vec3::new(1.0, 2.0, 3.0);
    let angular = AxisAngle::new(Vec3::Z, PI * 0.5);

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::from_rotation(Quat::from_axis_angle(Vec3::Z, 0.0)),
            Body::Sphere { radius: 1.0 },
            BodyType::Kinematic,
            Velocity::from_linear(linear).with_angular(angular),
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
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
