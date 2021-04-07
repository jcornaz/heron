#![cfg(feature = "2d")]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{Body, RotationConstraints};
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
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

    assert!(
        bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap()
            .effective_world_inv_inertia_sqrt
            > 0.0
    );
}

#[test]
fn rotation_can_be_locked_at_creation() {
    let mut app = test_app();

    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        RotationConstraints::lock(),
    ));

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();

    assert_eq!(
        bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap()
            .effective_world_inv_inertia_sqrt,
        0.0
    );
}

#[test]
fn rotation_can_be_locked_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn((GlobalTransform::default(), Body::Sphere { radius: 10.0 }));

    app.update();

    app.world
        .insert(entity, RotationConstraints::lock())
        .unwrap();

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();

    assert_eq!(
        bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap()
            .effective_world_inv_inertia_sqrt,
        0.0
    );
}

#[test]
fn rotation_is_unlocked_if_component_is_removed() {
    let mut app = test_app();

    let entity = app.world.spawn((
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
        RotationConstraints::lock(),
    ));

    app.update();

    app.world.remove_one::<RotationConstraints>(entity).unwrap();

    app.update();

    let bodies = app.resources.get::<RigidBodySet>().unwrap();

    assert!(
        bodies
            .get(app.world.get::<BodyHandle>(entity).unwrap().rigid_body())
            .unwrap()
            .effective_world_inv_inertia_sqrt
            > 0.0
    );
}
