#![cfg(any(dim2, dim3))]

use std::time::Duration;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, PhysicMaterial, PhysicsSteps, RigidBody};
use heron_rapier::convert::IntoRapier;
use heron_rapier::{ColliderHandle, RapierPlugin};
use utils::*;

mod utils;

fn test_app() -> App {
    let mut builder = App::new();
    builder
        .init_resource::<TypeRegistryArc>()
        .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)))
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin);
    builder
}

#[test]
fn restitution_can_be_defined_when_creating_body() {
    let mut app = test_app();

    let restitution = 0.42;
    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
            PhysicMaterial {
                restitution,
                ..Default::default()
            },
        ))
        .id();

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();

    assert_eq!(restitution, collider.restitution())
}

#[test]
fn restitution_can_be_updated() {
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

    let restitution = 2.0;
    app.world.entity_mut(entity).insert(PhysicMaterial {
        restitution,
        ..Default::default()
    });

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();

    assert_eq!(restitution, collider.restitution())
}
