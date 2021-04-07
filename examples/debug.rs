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
    commands.spawn(Camera2dBundle::default());

    // Sphere
    commands
        .spawn((Transform::default(), GlobalTransform::default()))
        .with(Body::Sphere { radius: 50.0 })
        .with(BodyType::Static);

    // Cuboid
    commands
        .spawn((
            Transform::from_translation(Vec3::unit_x() * 300.0),
            GlobalTransform::default(),
        ))
        .with(Body::Cuboid {
            half_extends: Vec2::new(50.0, 50.0).extend(0.0),
        })
        .with(BodyType::Static);

    // Capsule
    commands
        .spawn((
            Transform::from_translation(Vec3::unit_x() * -300.0),
            GlobalTransform::default(),
        ))
        .with(Body::Capsule {
            radius: 50.0,
            half_segment: 50.0,
        })
        .with(BodyType::Static);

    // ConvexHull, in this case describing a triangle
    commands
        .spawn((
            Transform::from_translation(Vec3::unit_y() * 150.0),
            GlobalTransform::default(),
        ))
        .with(Body::ConvexHull {
            points: vec![
                Vec3::new(0.0, -50.0, 0.0),
                Vec3::new(50.0, 0.0, 0.0),
                Vec3::new(-50.0, 0.0, 0.0),
            ],
        })
        .with(BodyType::Static);
}
