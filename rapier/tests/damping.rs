#![cfg(any(dim2, dim3))]

use std::time::Duration;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{Damping, PhysicsSteps, RigidBody};
use heron_rapier::RapierPlugin;
use utils::*;

mod utils;

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
fn body_is_created_with_damping() {
    let mut app = test_app();

    let linear = 0.5;
    let angular = 0.8;

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Damping { linear, angular },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(body.linear_damping(), linear);
    assert_eq!(body.angular_damping(), angular);
}

#[test]
fn damping_can_be_added_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((GlobalTransform::default(), RigidBody::Dynamic))
        .id();

    app.update();

    let linear = 0.5;
    let angular = 0.8;

    app.world
        .entity_mut(entity)
        .insert(Damping { linear, angular });

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(body.linear_damping(), linear);
    assert_eq!(body.angular_damping(), angular);
}

#[test]
fn damping_can_be_updated_after_creation() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Damping {
                linear: 0.2,
                angular: 0.2,
            },
        ))
        .id();

    app.update();

    let linear = 0.5;
    let angular = 0.8;

    let mut damping = app.world.get_mut::<Damping>(entity).unwrap();
    damping.linear = linear;
    damping.angular = angular;

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(body.linear_damping(), linear);
    assert_eq!(body.angular_damping(), angular);
}

#[test]
fn restore_damping_on_removal() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Damping {
                linear: 0.2,
                angular: 0.2,
            },
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<Damping>();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let body = bodies.get(*app.world.get(entity).unwrap()).unwrap();

    assert_eq!(
        body.linear_damping(),
        RigidBodyDamping::default().linear_damping
    );
    assert_eq!(
        body.angular_damping(),
        RigidBodyDamping::default().angular_damping
    );
}
