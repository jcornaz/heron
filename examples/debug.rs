use bevy::prelude::*;

use heron::*;

fn main() {
    App::build()
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
        .insert(RigidBody::Static);

    // Cuboid
    commands
        .spawn_bundle((
            Transform::from_translation(Vec3::X * 300.0),
            GlobalTransform::default(),
        ))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(50.0, 50.0).extend(0.0),
        })
        .insert(RigidBody::Static);

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
        .insert(RigidBody::Static);

    // ConvexHull, in this case describing a triangle
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
            ],
        })
        .insert(RigidBody::Static);
}
