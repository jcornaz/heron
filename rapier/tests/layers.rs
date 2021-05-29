#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionLayers, CollisionShape, PhysicsLayer, PhysicsSteps, RigidBody};
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;
use std::time::Duration;

enum TestLayer {
    A,
    B,
}

impl PhysicsLayer for TestLayer {
    fn to_bits(&self) -> u16 {
        match self {
            TestLayer::A => 1,
            TestLayer::B => 2,
        }
    }

    fn all_bits() -> u16 {
        3
    }
}

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
fn sets_the_collision_groups() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Sensor,
            CollisionShape::Sphere { radius: 1.0 },
            CollisionLayers::none()
                .with_group(TestLayer::A)
                .with_mask(TestLayer::B),
            GlobalTransform::default(),
        ))
        .id();

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(collider.collision_groups().0, (1 << 16) + 2)
}

#[test]
fn updates_the_collision_groups() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Sensor,
            CollisionShape::Sphere { radius: 1.0 },
            GlobalTransform::default(),
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).insert(
        CollisionLayers::none()
            .with_group(TestLayer::A)
            .with_mask(TestLayer::B),
    );

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(collider.collision_groups().0, (1 << 16) + 2)
}

#[test]
fn restore_the_collision_groups_on_removal() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Sensor,
            CollisionShape::Sphere { radius: 1.0 },
            CollisionLayers::none()
                .with_group(TestLayer::A)
                .with_mask(TestLayer::B),
            GlobalTransform::default(),
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<CollisionLayers>();

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(collider.collision_groups().0, u32::MAX)
}
