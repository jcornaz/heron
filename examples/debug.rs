use bevy::prelude::*;

use heron::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_startup_system(spawn.system())
        .run();
}

fn spawn(commands: &mut Commands) {
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
}
