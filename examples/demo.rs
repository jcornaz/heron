use std::f32::consts::PI;

use bevy::prelude::*;

use heron::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .insert_resource(Gravity::from(Vec2::new(0.0, -600.0))) // Define the gravity
        .add_startup_system(spawn)
        .add_system(log_collisions)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // The ground
    let size = Vec2::new(1000.0, 50.0);
    commands
        // Spawn a bundle that contains at least a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        })
        // Make it a rigid body
        .insert(RigidBody::Static)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        // Define restitution (so that it bounces)
        .insert(PhysicMaterial {
            restitution: 0.5,
            ..Default::default()
        });

    // The Ball
    let size = Vec2::new(30.0, 30.0);
    commands
        // Spawn a bundle that contains at least a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-400.0, 200.0, 0.0)),
            ..Default::default()
        })
        // Make it a rigid body
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        // Add an initial velocity. (it is also possible to read/mutate this component later)
        .insert(Velocity::from(Vec2::X * 300.0).with_angular(AxisAngle::new(Vec3::Z, -PI)))
        // Define restitution (so that it bounces)
        .insert(PhysicMaterial {
            restitution: 0.7,
            ..Default::default()
        });
}

fn log_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {
                println!("Collision started between {:?} and {:?}", d1, d2)
            }
            CollisionEvent::Stopped(d1, d2) => {
                println!("Collision stopped between {:?} and {:?}", d1, d2)
            }
        }
    }
}
