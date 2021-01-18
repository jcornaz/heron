use bevy::prelude::*;

use bevy::input::system::exit_on_esc_system;
use heron::*;
use heron_rapier::PhysicsPlugin;
use std::ops::DerefMut;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(spawn.system())
        .add_system(scale.system())
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

fn scale(inputs: Res<Input<KeyCode>>, mut query: Query<(&mut Body, &mut Transform)>) {
    let factor = if inputs.pressed(KeyCode::Up) {
        2.0
    } else if inputs.pressed(KeyCode::Down) {
        0.5
    } else {
        return;
    };

    for (mut body, mut transform) in query.iter_mut() {
        transform.scale *= factor;
        // if let Body::Sphere { radius } = body.deref_mut() {
        //     println!("Scaling by {}", factor);
        //     *radius *= factor;
        // }
    }
}
