#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{Body, PhysicMaterial};
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::rapier::geometry::ColliderSet;
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
fn restitution_can_be_defined_when_creating_body() {
    let mut app = test_app();

    let restitution = 0.42;
    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        PhysicMaterial {
            restitution,
            ..Default::default()
        },
    ));

    app.update();

    let colliders = app.resources.get::<ColliderSet>().unwrap();
    let collider = colliders
        .get(app.world.get::<BodyHandle>(entity).unwrap().collider())
        .unwrap();

    assert_eq!(restitution, collider.restitution)
}

#[test]
fn restitution_can_be_updated() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn((GlobalTransform::default(), Body::Sphere { radius: 10.0 }));

    app.update();

    let restitution = 2.0;
    app.world
        .insert_one(
            entity,
            PhysicMaterial {
                restitution,
                ..Default::default()
            },
        )
        .unwrap();

    app.update();

    let colliders = app.resources.get::<ColliderSet>().unwrap();
    let collider = colliders
        .get(app.world.get::<BodyHandle>(entity).unwrap().collider())
        .unwrap();

    assert_eq!(restitution, collider.restitution)
}
