#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, PhysicMaterial, RigidBody};
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;

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
fn friction_can_be_defined_when_creating_body() {
    let mut app = test_app();

    let friction = 0.5;
    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
            PhysicMaterial {
                friction,
                ..Default::default()
            },
        ))
        .id();

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(friction, collider.friction)
}

#[test]
fn friction_can_be_updated() {
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

    let friction = 0.1;
    app.world.entity_mut(entity).insert(PhysicMaterial {
        friction,
        ..Default::default()
    });

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(friction, collider.friction)
}
