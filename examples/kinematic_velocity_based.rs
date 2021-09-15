use bevy::prelude::*;

use heron::*;
use heron_core::Velocity;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_startup_system(spawn.system())
        .add_system_to_stage(CoreStage::Update, player_input.system())
        .run();
}

fn player_input(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Sprite>>
) {
    for (mut vel) in player_query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            vel.linear.y += 10.0;
        }
        if keys.pressed(KeyCode::S) {
            vel.linear.y -= 10.0;
        }
        if keys.pressed(KeyCode::D) {
            vel.linear.x += 10.0;
        }
        if keys.pressed(KeyCode::A) {
            vel.linear.x -= 10.0;
        }
    }
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let size = Vec2::new(30.0, 30.0);
    let mut player_entity = commands.spawn();
    player_entity
        .insert_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(Velocity::from_linear(Vec3::new(0.0, 0.0, 0.0)))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        });
}
