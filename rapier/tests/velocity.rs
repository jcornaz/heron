#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::*;
use heron_rapier::convert::IntoBevy;
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
                params.set_dt(1.0);
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

    assert_eq!(linear, (*body.linvel()).into_bevy());
    assert_eq!(angular, (*body.angvel()).into_bevy().into());
}

#[test]
#[ignore]
fn velocity_may_be_added_after_creating_the_body() {
    todo!()
}

#[test]
#[ignore]
fn velocity_is_updated_to_reflect_rapier_world() {
    todo!()
}
