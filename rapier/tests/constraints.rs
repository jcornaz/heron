#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{Body, PhysicMaterial};
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
use heron_rapier::rapier::math::AngVector;
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
fn rotation_is_not_constrained_without_the_component() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn((GlobalTransform::default(), Body::Sphere { radius: 10.0 }));

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();
    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    let inv_angular_inertia: Vec3 =
        vec3_from_angle_vector(body.mass_properties().inv_principal_inertia_sqrt);

    let inv_angular_inertia: Vec3 = inv_angular_inertia.into();

    #[cfg(feature = "3d")]
    assert!(inv_angular_inertia.x > 0.0);

    #[cfg(feature = "3d")]
    assert!(inv_angular_inertia.y > 0.0);

    assert!(inv_angular_inertia.z > 0.0);
}

#[cfg(feature = "2d")]
fn vec3_from_angle_vector(vector: AngVector<f32>) -> Vec3 {
    Vec3::unit_z() * vector
}

#[cfg(feature = "3d")]
fn vec3_from_angle_vector(vector: AngVector<f32>) -> Vec3 {
    vector.into_bevy()
}
