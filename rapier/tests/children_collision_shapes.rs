#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, RigidBody};
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::dynamics::{IntegrationParameters, RigidBodySet};
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
fn can_use_child_entity_for_the_collision_shape() {
    let mut app = test_app();

    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::from_axis_angle(Vec3::Z, 1.0);
    app.world
        .spawn()
        .insert_bundle((GlobalTransform::default(), RigidBody::Dynamic))
        .with_children(|children| {
            children.spawn_bundle((
                Transform {
                    translation,
                    rotation,
                    ..Default::default()
                },
                CollisionShape::Sphere { radius: 1.0 },
            ));
        });

    app.update();

    let bodies = app.world.get_resource::<RigidBodySet>().unwrap();
    let colliders = app.world.get_resource::<ColliderSet>().unwrap();

    assert_eq!(bodies.len(), 1);
    assert_eq!(colliders.len(), 1);

    let (body_handle, body) = bodies.iter().next().unwrap();
    let (collider_handle, collider) = colliders.iter().next().unwrap();

    assert_eq!(body.colliders(), &[collider_handle]);
    assert_eq!(body.position().into_bevy(), (Vec3::ZERO, Quat::IDENTITY));

    assert_eq!(collider.parent(), body_handle);

    let (actual_translation, actual_rotation) = collider.position_wrt_parent().into_bevy();

    #[cfg(feature = "2d")]
    assert_eq!(
        actual_translation,
        Vec3::new(translation.x, translation.y, 0.0)
    );
    #[cfg(feature = "3d")]
    assert_eq!(actual_translation, translation);

    assert!((actual_rotation.x - rotation.x).abs() < 0.00001);
    assert!((actual_rotation.y - rotation.y).abs() < 0.00001);
    assert!((actual_rotation.z - rotation.z).abs() < 0.00001);
    assert!((actual_rotation.w - rotation.w).abs() < 0.00001);
}

#[test]
fn can_change_the_position_of_a_shape_inserted_in_child_entity() {
    let mut app = test_app();

    let translation = Vec3::new(1.0, 2.0, 3.0);

    let shape_entity = app
        .world
        .spawn()
        .insert_bundle((Transform::default(), CollisionShape::Sphere { radius: 1.0 }))
        .id();

    app.world
        .spawn()
        .insert_bundle((GlobalTransform::default(), RigidBody::Dynamic))
        .push_children(&[shape_entity]);

    app.update();

    app.world
        .get_mut::<Transform>(shape_entity)
        .unwrap()
        .translation = Vec3::new(1.0, 2.0, 3.0);

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();

    let (_, collider) = colliders.iter().next().unwrap();

    let (actual_translation, _) = collider.position_wrt_parent().into_bevy();

    #[cfg(feature = "2d")]
    assert_eq!(
        actual_translation,
        Vec3::new(translation.x, translation.y, 0.0)
    );
    #[cfg(feature = "3d")]
    assert_eq!(actual_translation, translation);
}

#[test]
fn updating_local_transform_of_a_rigid_body_doesnt_affect_the_shape() {
    let mut app = test_app();

    let entity = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 1.0 },
        ))
        .id();

    app.update();

    app.world.get_mut::<Transform>(entity).unwrap().translation = Vec3::new(1.0, 2.0, 3.0);

    app.update();

    let colliders = app.world.get_resource::<ColliderSet>().unwrap();

    let (_, collider) = colliders.iter().next().unwrap();

    let (actual_translation, _) = collider.position_wrt_parent().into_bevy();

    assert_eq!(actual_translation, Vec3::default());
}
