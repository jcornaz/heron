#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionLayers, CollisionShape, Layer, RigidBody};
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;

enum TestLayer {
    A,
    B,
}

impl Layer for TestLayer {
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
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin {
            step_per_second: None,
            parameters: IntegrationParameters::default(),
        });
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
