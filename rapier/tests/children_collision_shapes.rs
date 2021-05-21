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

    app.world
        .spawn()
        .insert_bundle((GlobalTransform::default(), RigidBody::Dynamic))
        .with_children(|children| {
            children.spawn_bundle((
                Transform {
                    translation: Vec3::new(1.0, 2.0, 3.0),
                    rotation: Quat::from_axis_angle(Vec3::Z, 1.0),
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

    #[cfg(feature = "2d")]
    assert_eq!(
        collider.position_wrt_parent().into_bevy(),
        (
            Vec3::new(1.0, 2.0, 0.0),
            Quat::from_axis_angle(Vec3::Z, 1.0)
        )
    );

    #[cfg(feature = "3d")]
    assert_eq!(
        collider.position_wrt_parent().into_bevy(),
        (
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_axis_angle(Vec3::Z, 1.0)
        )
    );
}
