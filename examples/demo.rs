use std::ops::DerefMut;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;

use heron::*;
use heron_rapier::PhysicsPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(spawn.system())
        .add_system(scale.system())
        .add_system(delete.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn spawn(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default()).spawn((
        Body::Sphere { radius: 50.0 },
        Transform::default(),
        GlobalTransform::default(),
    ));
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
        let Body::Sphere { radius } = body.deref_mut();
        *radius = lerp(*radius, *radius * factor, time.delta_seconds());
    }
}

fn delete(inputs: Res<Input<KeyCode>>, query: Query<Entity, With<Body>>, commands: &mut Commands) {
    if inputs.pressed(KeyCode::Delete) {
        for entity in query.iter() {
            commands.despawn(entity);
        }
    }
}

fn lerp(start: f32, end: f32, factor: f32) -> f32 {
    start + ((end - start) * factor)
}
