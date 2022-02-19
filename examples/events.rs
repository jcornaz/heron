use bevy::prelude::*;

use heron::prelude::*;

const SPEED: f32 = 300.0;

#[derive(PhysicsLayer)]
enum Layer {
    Enemy,
    Player,
}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_system(handle_input)
        .add_system(log_collisions)
        .add_system(kill_enemy)
        .run();
}

// ANCHOR: log-collisions
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
// ANCHOR_END: log-collisions

// ANCHOR: kill-enemy
fn kill_enemy(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    events
        .iter()
        // We care about when the entities "start" to collide
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_player(layers_1) && is_enemy(layers_2) {
                Some(entity_2)
            } else if is_player(layers_2) && is_enemy(layers_1) {
                Some(entity_1)
            } else {
                // This event is not the collision between an enemy and the player. We can ignore it.
                None
            }
        })
        .for_each(|enemy_entity| commands.entity(enemy_entity).despawn());
}

// Note: We check both layers each time to avoid a false-positive
// that can occur if an entity has the default (unconfigured) `CollisionLayers`
fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player) && !layers.contains_group(Layer::Enemy)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Enemy)
}
// ANCHOR_END: kill-enemy

fn spawn_player(mut commands: Commands) {
    let size = Vec2::new(30.0, 30.0);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-400.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(Velocity::default())
        .insert(CollisionLayers::new(Layer::Player, Layer::Enemy));
}

fn spawn_enemy(mut commands: Commands) {
    let size = Vec2::new(30.0, 30.0);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(CollisionLayers::new(Layer::Enemy, Layer::Player));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn handle_input(input: Res<Input<KeyCode>>, mut players: Query<&mut Velocity, With<Player>>) {
    let x = if input.pressed(KeyCode::Left) {
        -1.0
    } else if input.pressed(KeyCode::Right) {
        1.0
    } else {
        0.0
    };

    let y = if input.pressed(KeyCode::Down) {
        -1.0
    } else if input.pressed(KeyCode::Up) {
        1.0
    } else {
        0.0
    };

    let target_velocity = Vec2::new(x, y).normalize_or_zero().extend(0.0) * SPEED;

    for mut velocity in players.iter_mut() {
        velocity.linear = target_velocity;
    }
}
