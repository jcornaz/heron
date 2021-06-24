#![cfg(any(dim2, dim3))]
use std::time::Duration;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::utils::NearZero;
use heron_core::{CollisionShape, PhysicMaterial, PhysicsSteps, RigidBody};
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::dynamics::{MassProperties, RigidBodySet};
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)))
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin);
    builder.app
}

#[test]
fn bodies_are_created_with_a_default_density() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();
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
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
            PhysicMaterial {
                density: 2.0,
                ..Default::default()
            },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(body.mass_properties(), &MassProperties::from_ball(2.0, 1.0));
}

#[test]
fn density_can_be_updated_after_creation() {
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

    app.world.entity_mut(entity).insert(PhysicMaterial {
        density: 2.0,
        ..Default::default()
    });

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(body.mass_properties(), &MassProperties::from_ball(2.0, 1.0));
}
