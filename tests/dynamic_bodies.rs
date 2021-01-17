#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use std::f32::consts::PI;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron::rapier::dynamics::RigidBodySet;
use heron::rapier::geometry::ColliderSet;
use heron::*;

struct TestEntity;

#[test]
fn creates_body_in_rapier_world() {
    let mut app = {
        let mut builder = App::build();
        builder
            .init_resource::<TypeRegistryArc>()
            .add_plugin(CorePlugin)
            .add_plugin(PhysicsPlugin::default());
        builder.app
    };

    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::from_axis_angle(Vec3::unit_z(), PI);

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

    assert_eq!(actual_rotation, rotation);
}
