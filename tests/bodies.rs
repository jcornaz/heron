#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use std::f32::consts::PI;
use std::ops::DerefMut;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron::rapier::dynamics::RigidBodySet;
use heron::rapier::geometry::ColliderSet;
use heron::*;

struct TestEntity;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(PhysicsPlugin::default());
    builder.app
}

#[test]
fn creates_body_in_rapier_world() {
    let mut app = test_app();

    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::from_axis_angle(Vec3::unit_z(), PI / 2.0);

    let entity = app.world.spawn((
        TestEntity,
        Body::Sphere { radius: 2.0 },
        GlobalTransform {
            translation,
            rotation,
            ..Default::default()
        },
    ));

    app.update();

    let handle = app
        .world
        .get::<BodyHandle>(entity)
        .expect("No body handle attached");

    let bodies = app.resources.get::<RigidBodySet>().unwrap();
    let colliders = app.resources.get::<ColliderSet>().unwrap();

    let body = bodies
        .get(handle.rigid_body())
        .expect("No rigid body referenced by the handle");

    let collider = colliders
        .get(handle.collider())
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

    let (actual_translation, actual_rotation) = convert::from_isometry(*body.position());

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

    let entity = app.world.spawn((
        TestEntity,
        Body::Sphere { radius: 2.0 },
        GlobalTransform::default(),
    ));

    app.update();

    let mut body_def = app.world.get_mut::<Body>(entity).unwrap();
    let Body::Sphere { radius } = body_def.deref_mut();
    *radius = 42.0;

    app.update();

    let colliders = app.resources.get::<ColliderSet>().unwrap();
    let handle = app.world.get::<BodyHandle>(entity).unwrap();
    let collider = colliders.get(handle.collider()).unwrap();

    assert_eq!(collider.shape().as_ball().unwrap().radius, 42.0)
}

#[test]
fn update_transform() {
    let mut app = test_app();

    let entity = app.world.spawn((
        TestEntity,
        Body::Sphere { radius: 2.0 },
        GlobalTransform::default(),
    ));

    app.update();

    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::from_axis_angle(Vec3::unit_z(), PI / 2.0);

    let mut transform = app.world.get_mut::<GlobalTransform>(entity).unwrap();
    transform.translation = translation;
    transform.rotation = rotation;

    app.update();

    let colliders = app.resources.get::<RigidBodySet>().unwrap();
    let handle = app.world.get::<BodyHandle>(entity).unwrap();
    let rigid_body = colliders.get(handle.rigid_body()).unwrap();
    let (actual_translation, actual_rotation) = convert::from_isometry(*rigid_body.position());

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

    let entity = app.world.spawn((
        TestEntity,
        Body::Sphere { radius: 2.0 },
        GlobalTransform::default(),
    ));

    app.update();

    app.world.remove_one::<Body>(entity).unwrap();
    app.update();

    assert!(app.world.get::<BodyHandle>(entity).is_err());

    let bodies = app.resources.get::<RigidBodySet>().unwrap();
    assert_eq!(bodies.len(), 0);

    let colliders = app.resources.get::<ColliderSet>().unwrap();
    assert_eq!(colliders.len(), 0);
}

#[test]
#[ignore]
fn remove_body_entity() {
    todo!()
}
