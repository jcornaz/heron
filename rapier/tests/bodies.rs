#![cfg(any(dim2, dim3))]

use std::convert::From;
use std::f32::consts::PI;
use std::ops::DerefMut;
use std::time::Duration;

use bevy::core::CorePlugin;
use bevy::math::Affine3A;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, PhysicsSteps, PhysicsTime, RigidBody};
use heron_rapier::convert::{IntoBevy, IntoRapier};
use heron_rapier::{ColliderHandle, RapierPlugin, RigidBodyHandle};
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
fn creates_body_in_rapier_world() {
    let mut app = test_app();

    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::from_axis_angle(Vec3::Z, PI / 2.0);

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 2.0 },
            GlobalTransform::from(Affine3A::from_rotation_translation(rotation, translation)),
        ))
        .id();

    app.update();

    let bodies = app.world.resource::<RigidBodySet>();
    let colliders = app.world.resource::<ColliderSet>();

    let body = bodies
        .get(
            app.world
                .get::<RigidBodyHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .expect("No rigid body referenced by the handle");

    let collider = colliders
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .expect("No collider referenced by the handle");

    assert_eq!(
        Entity::from_bits(body.user_data as u64),
        entity,
        "Entity not referenced in body user_data"
    );
    assert_eq!(
        Entity::from_bits(collider.user_data as u64),
        entity,
        "Entity not referenced in collider user_data"
    );

    let shape = collider.shape().as_ball().expect("The shape is not a ball");

    assert_eq!(shape.radius, 2.0); // (The radius should be scaled)

    let (actual_translation, actual_rotation) = body.position().into_bevy();

    #[cfg(dim3)]
    assert_eq!(actual_translation, translation);

    #[cfg(dim2)]
    assert_eq!(actual_translation.truncate(), translation.truncate());

    let (axis, angle) = rotation.to_axis_angle();
    let (actual_axis, actual_angle) = actual_rotation.to_axis_angle();

    assert!(actual_axis.angle_between(axis) < 0.001);
    assert!((actual_angle - angle).abs() < 0.001);
}

#[test]
fn update_shape() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 2.0 },
            GlobalTransform::default(),
        ))
        .id();

    {
        app.update();

        let mut body_def = app.world.get_mut::<CollisionShape>(entity).unwrap();
        if let CollisionShape::Sphere { radius } = body_def.deref_mut() {
            *radius = 42.0;
        }
    }

    app.update();

    let colliders = app.world.resource::<ColliderSet>();
    let collider = colliders
        .get(
            app.world
                .get::<ColliderHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();

    assert_eq!(collider.shape().as_ball().unwrap().radius, 42.0)
}

#[test]
fn update_rapier_position() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 2.0 },
            GlobalTransform::default(),
        ))
        .id();

    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::from_axis_angle(Vec3::Z, PI / 2.0);

    {
        app.update();
        let mut transform = app.world.get_mut::<GlobalTransform>(entity).unwrap();
        *transform =
            GlobalTransform::from(Affine3A::from_rotation_translation(rotation, translation));
    }

    app.update();

    let colliders = app.world.resource::<RigidBodySet>();
    let rigid_body = colliders
        .get(
            app.world
                .get::<RigidBodyHandle>(entity)
                .unwrap()
                .into_rapier(),
        )
        .unwrap();
    let (actual_translation, actual_rotation) = rigid_body.position().into_bevy();

    #[cfg(dim3)]
    assert_eq!(actual_translation, translation);

    #[cfg(dim2)]
    assert_eq!(actual_translation.truncate(), translation.truncate());

    let (axis, angle) = rotation.to_axis_angle();
    let (actual_axis, actual_angle) = actual_rotation.to_axis_angle();

    assert!(actual_axis.angle_between(axis) < 0.001);
    assert!((actual_angle - angle).abs() < 0.001);
}

#[test]
fn remove_body_component() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 2.0 },
            GlobalTransform::default(),
        ))
        .id();

    app.update();

    app.world.entity_mut(entity).remove::<RigidBody>();
    app.update();

    assert!(app.world.get::<RigidBodyHandle>(entity).is_none());
    assert!(app.world.get::<ColliderHandle>(entity).is_none());

    let bodies = app.world.resource::<RigidBodySet>();
    assert_eq!(bodies.len(), 0);

    let colliders = app.world.resource::<ColliderSet>();
    assert_eq!(colliders.len(), 0);
}

#[test]
fn despawn_body_entity() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 2.0 },
            GlobalTransform::default(),
        ))
        .id();

    app.update();

    app.world.despawn(entity);
    app.update();

    assert!(app.world.get::<RigidBodyHandle>(entity).is_none());
    assert!(app.world.get::<ColliderHandle>(entity).is_none());

    let bodies = app.world.resource::<RigidBodySet>();
    assert_eq!(bodies.len(), 0);

    let colliders = app.world.resource::<ColliderSet>();
    assert_eq!(colliders.len(), 0);
}

#[test]
fn despawn_body_while_paused() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 2.0 },
            GlobalTransform::default(),
        ))
        .id();

    app.update();
    app.world.resource_mut::<PhysicsTime>();
    app.world.despawn(entity);
    app.update();
    app.world.resource_mut::<PhysicsTime>().resume();
    app.update();

    let bodies = app.world.resource::<RigidBodySet>();
    assert_eq!(bodies.len(), 0);

    let colliders = app.world.resource::<ColliderSet>();
    assert_eq!(colliders.len(), 0);
}
