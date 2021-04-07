#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::prelude::{GlobalTransform, Transform};
use bevy::reflect::TypeRegistryArc;
use heron_core::{Acceleration, AxisAngle, Body};
use heron_rapier::convert::IntoBevy;

#[cfg(feature = "3d")]
use heron_rapier::rapier::math::Vector;
use heron_rapier::{
    rapier::dynamics::{IntegrationParameters, RigidBodySet},
    BodyHandle, RapierPlugin,
};

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
fn body_is_created_with_acceleration() {
    let mut app = test_app();

    #[cfg(feature = "3d")]
    let linear = Vec3::new(1.0, 2.0, 3.0);
    #[cfg(feature = "2d")]
    let linear = Vec3::new(1.0, 2.0, 0.0);

    let angular = AxisAngle::new(Vec3::unit_z(), 1.0);

    let entity = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Body::Sphere { radius: 1.0 },
        Acceleration { linear, angular },
    ));

    app.update();

    {
        let bodies = app.resources.get::<RigidBodySet>().unwrap();

        let body = bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap();

        println!("{:?}", body);
        assert_eq!(body.linvel().into_bevy(), Vec3::zero());
        assert_eq_angular(body.angvel(), AxisAngle::from(Vec3::zero()));
    }

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();

    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    println!("{:?}", body);
    assert_eq!(body.linvel().into_bevy(), linear);
    assert_eq_angular(body.angvel(), angular);
}

#[test]
fn acceleration_may_be_added_after_creating_the_body() {
    let mut app = test_app();

    let entity = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Body::Sphere { radius: 1.0 },
    ));

    app.update();

    #[cfg(feature = "3d")]
    let linear = Vec3::new(1.0, 2.0, 3.0);
    #[cfg(feature = "2d")]
    let linear = Vec3::new(1.0, 2.0, 0.0);

    let angular = AxisAngle::new(Vec3::unit_z(), 2.0);

    app.world
        .insert(entity, Acceleration { linear, angular })
        .unwrap();

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();

    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    assert_eq!(body.linvel().into_bevy(), linear);
    assert_eq_angular(body.angvel(), angular);
}

#[cfg(feature = "3d")]
fn assert_eq_angular(actual: &Vector<f32>, expected: AxisAngle) {
    assert_eq!(actual.into_bevy(), expected.into());
}

#[cfg(feature = "2d")]
fn assert_eq_angular(expected: f32, actual: AxisAngle) {
    assert!(
        (expected - actual.angle()).abs() < 0.00001,
        "actual rapier angle ({}) doesn't match expected axis-angle: {:?}",
        expected,
        actual
    );
}
