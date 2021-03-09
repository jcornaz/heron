#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{Body, PhysicMaterial, RotationConstraints};
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
#[cfg(feature = "2d")]
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
#[cfg(feature = "2d")]
fn rotation_can_be_locked_on_creation() {
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
