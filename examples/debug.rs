use bevy::prelude::*;

use heron::*;

fn main() {
    App::new()
        .insert_resource(Gravity::from(Vec3::new(0., -98.1, 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_startup_system(spawn.system())
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Sphere
    commands
        .spawn_bundle((Transform::default(), GlobalTransform::default()))
        .insert(CollisionShape::Sphere { radius: 50.0 })
        .insert(RigidBody::Sensor);

    // Cuboid
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::X * 300.0),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(50.0, 50.0).extend(0.0),
            border_radius: None,
        })
        .insert(RigidBody::KinematicVelocityBased);

    // Capsule
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::X * -300.0),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::Capsule {
            radius: 50.0,
            half_segment: 50.0,
        })
        .insert(RigidBody::KinematicPositionBased);

    // ConvexHull, a random quadrilateral
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::Y * 150.0),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(0.0, -50.0, 0.0),
                Vec3::new(50.0, 0.0, 0.0),
                Vec3::new(-50.0, 0.0, 0.0),
                Vec3::new(5.0, 10.0, 0.0),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Dynamic);

    // Height field
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::Y * -200.0),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::HeightField {
            size: Vec2::new(700.0, 0.0),
            heights: vec![vec![50.0, 0.0, 10.0, 30.0, 20.0, 0.0, 20.0]],
        })
        .insert(RigidBody::Static);
}
