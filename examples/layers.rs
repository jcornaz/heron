use bevy::prelude::*;

use heron::*;

// ANCHOR: layer-enum
#[derive(Layer)]
enum Layer {
    World,
    Player,
}
// ANCHOR_END: layer-enum

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .insert_resource(Gravity::from(Vec2::new(0.0, -600.0))) // Define the gravity
        .add_startup_system(spawn.system())
        .run();
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let size = Vec2::new(1000.0, 50.0);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
        })
        // ANCHOR: layer-component-world
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player),
        );
    // ANCHOR_END: layer-component-world

    let size = Vec2::new(30.0, 30.0);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
        })
        // ANCHOR: layer-component-player
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Player)
                .with_mask(Layer::World),
        );
    // ANCHOR_END: layer-component-player
}
