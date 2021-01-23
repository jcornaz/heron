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
#[ignore]
fn body_with_linear_velocity_is_moved() {
    let mut app = test_app();

    let entity = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Body::Sphere { radius: 1.0 },
        LinearVelocity::from(Vec3::new(1.0, 2.0, 3.0)),
    ));

    app.update();

    let position = app
        .resources
        .get::<RigidBodySet>()
        .unwrap()
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap()
        .position()
        .translation
        .into_bevy();

    assert!(is_near(position.x, 1.0));
    assert!(is_near(position.y, 2.0));

    #[cfg(feature = "3d")]
    assert!(is_near(position.z, 3.0));

    app.update();

    let position = app
        .resources
        .get::<RigidBodySet>()
        .unwrap()
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap()
        .position()
        .translation
        .into_bevy();

    assert!(is_near(position.x, 2.0));
    assert!(is_near(position.y, 4.0));

    #[cfg(feature = "3d")]
    assert!(is_near(position.z, 6.0));
}

#[test]
#[ignore]
fn body_with_angular_velocity_is_rotated() {
    todo!()
}

#[test]
#[ignore]
fn non_body_with_linear_velocity_is_moved() {
    todo!()
}

#[test]
#[ignore]
fn non_body_with_angular_velocity_is_rotated() {
    todo!()
}

#[test]
#[ignore]
fn velocity_is_updated_to_reflect_rapier_world() {
    todo!()
}

#[inline]
fn is_near(v1: f32, v2: f32) -> bool {
    (v2 - v1).abs() <= f32::EPSILON
}
