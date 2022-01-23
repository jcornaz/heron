#![cfg(feature = "2d")]

use bevy::prelude::*;

use heron_core::{CollisionShape, RigidBody};
use heron_debug::DebugPlugin;
use heron_rapier::RapierPlugin;
use rstest::*;

#[fixture]
fn app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(RapierPlugin::default())
        .add_plugin(DebugPlugin::default());
    app
}

#[rstest]
fn does_not_crash_after_despawn(mut app: App) {
    let entity = app
        .world
        .spawn()
        .insert_bundle((
            RigidBody::Dynamic,
            CollisionShape::default(),
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id();
    app.update();
    app.world.entity_mut(entity).despawn_recursive();
    app.update();
}
