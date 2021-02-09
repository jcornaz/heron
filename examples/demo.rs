use std::ops::DerefMut;

use bevy::prelude::*;

use heron::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(spawn.system())
        .add_system(scale.system())
        .add_system(delete.system())
        .add_system(apply_velocity.system())
        .run();
}

fn spawn(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            material: materials.add(Color::WHITE.into()),
            ..Default::default()
        })
        .with(Body::Cuboid {
            half_extends: Vec3::new(50.0, 50.0, 0.0),
        })
        .with(Velocity::from_linear(Vec3::unit_x()));
}

fn apply_velocity(inputs: Res<Input<KeyCode>>, mut query: Query<&mut Velocity>) {
    let linear = Vec3::unit_x()
        * if inputs.pressed(KeyCode::Left) {
            -1000.0
        } else if inputs.pressed(KeyCode::Right) {
            1000.0
        } else {
            0.0
        };

    for mut velocity in query.iter_mut() {
        velocity.linear = linear;
    }
}

fn scale(inputs: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut Body>) {
    let factor = if inputs.pressed(KeyCode::Up) {
        2.0
    } else if inputs.pressed(KeyCode::Down) {
        0.5
    } else {
        return;
    };

    for mut body in query.iter_mut() {
        if let Body::Cuboid { half_extends } = body.deref_mut() {
            *half_extends = half_extends.lerp(*half_extends * factor, time.delta_seconds());
        }
    }
}

fn delete(inputs: Res<Input<KeyCode>>, query: Query<Entity, With<Body>>, commands: &mut Commands) {
    if inputs.pressed(KeyCode::Delete) {
        for entity in query.iter() {
            commands.despawn(entity);
        }
    }
}
