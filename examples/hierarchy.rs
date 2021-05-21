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

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(30.0, 60.0)),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(-400.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .with_children(|children| {
            // The collision shape may be added to a child entity of the rigid body.
            // That makes possible to add multiple collision shapes for the rigid body and/or to define an offset for the collision shape
            children.spawn_bundle((
                Transform::from_translation(Vec3::Y * 15.0),
                CollisionShape::Cuboid {
                    half_extends: Vec2::new(30.0, 30.0).extend(0.0) / 2.0,
                },
                GlobalTransform::default(),
            ));
        })
        .insert(Velocity::from(Vec2::X * 150.0).with_angular(AxisAngle::new(Vec3::Z, PI * -0.7)))
        .insert(PhysicMaterial {
            restitution: 0.7,
            friction: 0.1,
            ..Default::default()
        });
}

fn spawn_ground_and_camera(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let size = Vec2::new(1500.0, 50.0);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
        })
        .insert(PhysicMaterial {
            restitution: 0.5,
            ..Default::default()
        });
}
