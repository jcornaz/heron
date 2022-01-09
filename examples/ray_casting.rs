use bevy::prelude::*;
use heron::{
    rapier_plugin::{PhysicsWorld, ShapeCastCollisionType},
    *,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            height: 900.0,
            width: 900.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the plugin
        .add_startup_system(setup.system())
        .add_system(ray_cast_from_center.system())
        .add_system(shape_cast_from_center.system())
        .add_system(move_targeter.system())
        .run();
}

/// Marker struct for our targeter
#[derive(Component)]
struct Targeter;

/// Marker
#[derive(Component)]
struct ShapeCastIgnored;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn some walls to cast rays at
    for (size, pos) in &[
        (Vec2::new(800.0, 2.0), Vec3::new(0.0, -300.0, 0.0)),
        (Vec2::new(800.0, 2.0), Vec3::new(0.0, 300.0, 0.0)),
        (Vec2::new(2.0, 800.0), Vec3::new(-300.0, 0.0, 0.0)),
        (Vec2::new(2.0, 800.0), Vec3::new(300.0, 0.0, 0.0)),
        (Vec2::new(2.0, 100.0), Vec3::new(200.0, 0.0, 0.0)),
        (Vec2::new(2.0, 100.0), Vec3::new(-200.0, 0.0, 0.0)),
        (Vec2::new(100.0, 2.0), Vec3::new(0.0, 100.0, 0.0)),
        (Vec2::new(100.0, 2.0), Vec3::new(0.0, -100.0, 0.0)),
    ] {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(*size),
                    ..Default::default()
                },
                transform: Transform::from_translation(*pos),
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: size.extend(0.0) / 2.0,
                border_radius: None,
            });
    }

    // Spawn the targeter that the player can move
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 150., 1.),
            ..Default::default()
        })
        .insert(Targeter);

    // Spawn marker to show the center of the world, where the ray will come from
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(10., 10.)),
            ..Default::default()
        },
        transform: Transform::default(),
        ..Default::default()
    });

    // Spawn a shape that has a collision, but that also has an `ShapeCastIgnored` component that we
    // will use to filter it out from the shape cast below.
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(100., 100.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(200., 200., 0.),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(50., 50., 0.),
            border_radius: None,
        })
        .insert(ShapeCastIgnored);
}

/// This system will listen for `R` key presses and it will cast a ray from the center of the
/// world in the direction of the targeter
fn ray_cast_from_center(
    mut commands: Commands,
    physics_world: PhysicsWorld,
    mut targeter: Query<&Transform, With<Targeter>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::R) {
        return;
    }

    let targeter_transform = if let Ok(transform) = targeter.get_single_mut() {
        transform
    } else {
        return;
    };

    let world_center = Vec3::default();

    // Cast a ray from the center of the world, to the targeter
    let result = physics_world.ray_cast(
        world_center,
        targeter_transform.translation - world_center,
        true,
    );

    // If the ray hit anything between the world center and the targeter
    if let Some(collision_info) = dbg!(result) {
        // Spawn a green block at the collision point
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            },
            transform: Transform::from_translation(collision_info.collision_point),
            ..Default::default()
        });
    }
}

/// This system will listen for `S` key presses and it will cast a square shape  from the center of
/// the world in the direction of the targeter
fn shape_cast_from_center(
    mut commands: Commands,
    physics_world: PhysicsWorld,
    ignored_entities: Query<(), With<ShapeCastIgnored>>,
    mut targeter: Query<&Transform, With<Targeter>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::S) {
        return;
    }

    let targeter_transform = if let Ok(transform) = targeter.get_single_mut() {
        transform
    } else {
        return;
    };

    let world_center = Vec3::default();

    let shape_size = Vec2::new(30., 30.);

    let shape = CollisionShape::Cuboid {
        half_extends: shape_size.extend(0.) / 2.,
        border_radius: None,
    };

    // Cast a ray from the center of the world, to the targeter.
    let result = physics_world.shape_cast_with_filter(
        &shape,
        world_center,
        Quat::IDENTITY,
        targeter_transform.translation - world_center,
        // Collision layers can be used to do group-based filtering on ray/shape-casts. See
        // `layers.rs` example for more info. The default doesn't filter out any collisions.
        CollisionLayers::default(),
        // We can also do fine-grained filtering using a closure. In this case, only shape cast to
        // entities that don't have the `ShapeCastIgnored` component
        |entity| ignored_entities.get(entity).is_err(),
    );

    // If the ray hit anything between the world center and the targeter
    if let Some(collision) = dbg!(result) {
        if let ShapeCastCollisionType::Collided(info) = collision.collision_type {
            // Spawn a green block at the collision point
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(shape_size),
                    ..Default::default()
                },
                transform: Transform::from_translation(info.self_end_position),
                ..Default::default()
            });
        }
    }
}

/// System to move the targeter when the user presses the arrow keys
fn move_targeter(
    keyboard_input: Res<Input<KeyCode>>,
    mut targeter: Query<&mut Transform, With<Targeter>>,
) {
    const SPEED: f32 = 5.;

    let mut transform = if let Ok(transform) = targeter.get_single_mut() {
        transform
    } else {
        return;
    };

    let mut direction = Vec3::new(0., 0., 0.);

    if keyboard_input.pressed(KeyCode::Left) {
        direction += Vec3::new(-SPEED, 0., 0.);
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::new(SPEED, 0., 0.);
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction += Vec3::new(0., SPEED, 0.);
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction += Vec3::new(0., -SPEED, 0.);
    }

    transform.translation += direction;
}
