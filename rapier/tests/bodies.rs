#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use std::f32::consts::PI;
use std::ops::DerefMut;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, RigidBody};
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodyHandle, RigidBodySet};
use heron_rapier::rapier::geometry::{ColliderHandle, ColliderSet};
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
            GlobalTransform {
                translation,
                rotation,
                ..Default::default()
            },
        ))
        .id();

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let colliders = app.world.get_resource::<ColliderSet>().unwrap();

    let body = bodies
        .get(*app.world.get(entity).unwrap())
        .expect("No rigid body referenced by the handle");

    let collider = colliders
        .get(*app.world.get(entity).unwrap())
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

    #[cfg(feature = "3d")]
    assert_eq!(actual_translation, translation);

    #[cfg(feature = "2d")]
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

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    let collider = colliders.get(*app.world.get(entity).unwrap()).unwrap();

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
        transform.translation = translation;
        transform.rotation = rotation;
    }

    app.update();

    let colliders = app.world.get_resource::<RigidBodySet>().unwrap();
    let rigid_body = colliders.get(*app.world.get(entity).unwrap()).unwrap();
    let (actual_translation, actual_rotation) = rigid_body.position().into_bevy();

    #[cfg(feature = "3d")]
    assert_eq!(actual_translation, translation);

    #[cfg(feature = "2d")]
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

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    assert_eq!(bodies.len(), 0);

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
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

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    assert_eq!(bodies.len(), 0);

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();
    assert_eq!(colliders.len(), 0);
}
