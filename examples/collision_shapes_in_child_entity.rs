use std::f32::consts::PI;

use bevy::prelude::*;

use heron::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec2::new(0.0, -600.0)))
        .add_startup_system(spawn_ground_and_camera.system())
        .add_startup_system(spawn.system())
        .run();
}

fn spawn(mut commands: Commands) {
    let mut rigid_body_entity = commands.spawn();

    // ANCHOR: add-child-shape
    rigid_body_entity
        // The rigid body
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            children.spawn_bundle((
                // Position of the shape relative to its parent
                Transform::from_translation(Vec3::Y * 15.0),
                CollisionShape::Cuboid {
                    half_extends: Vec3::new(15.0, 15.0, 0.0),
                },
                GlobalTransform::default(),
            ));

            children.spawn_bundle((
                // Position of the shape relative to its parent
                Transform::from_translation(Vec3::X * 100.0),
                CollisionShape::Cuboid {
                    half_extends: Vec3::new(50.0, 50.0, 0.0),
                },
                GlobalTransform::default(),
            ));
        });
    // ANCHOR_END: add-child-shape

    rigid_body_entity
        .insert(Transform::from_translation(Vec3::new(-400.0, 200.0, 0.0)))
        .insert(GlobalTransform::default())
        .insert(Velocity::from(Vec2::X * 150.0).with_angular(AxisAngle::new(Vec3::Z, PI * -0.7)))
        .insert(PhysicMaterial {
            restitution: 0.7,
            friction: 0.1,
            ..Default::default()
        });
}

fn spawn_ground_and_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle((
        Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
        GlobalTransform::default(),
        RigidBody::Static,
        CollisionShape::Cuboid {
            half_extends: Vec2::new(1500.0, 50.0).extend(0.0) / 2.0,
        },
    ));
}
