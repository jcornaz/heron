#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::utils::NearZero;
use heron_core::{Body, PhysicMaterial};
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::dynamics::{IntegrationParameters, MassProperties, RigidBodySet};
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
fn bodies_are_created_with_a_default_density() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((GlobalTransform::default(), Body::Sphere { radius: 10.0 }))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();
    assert!(body.mass() > 0.0);

    let center: Vec3 = body.mass_properties().local_com.coords.into_bevy();
    assert!(center.is_near_zero())
}

#[test]
fn bodies_are_created_with_defined_density() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            Body::Sphere { radius: 1.0 },
            PhysicMaterial {
                density: 2.0,
                ..Default::default()
            },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    assert_eq!(body.mass_properties(), &MassProperties::from_ball(2.0, 1.0));
}

#[test]
fn density_can_be_updated_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((GlobalTransform::default(), Body::Sphere { radius: 1.0 }))
        .id();

    app.update();

    app.world.entity_mut(entity).insert(PhysicMaterial {
        density: 2.0,
        ..Default::default()
    });

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies
        .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
        .unwrap();

    assert_eq!(body.mass_properties(), &MassProperties::from_ball(2.0, 1.0));
}
