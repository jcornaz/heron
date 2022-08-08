use bevy::prelude::*;
use heron::prelude::*;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the Heron plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0))) // Define gravity
        .add_startup_system(spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    // Ensure we can see things
    commands.spawn_bundle(Camera2dBundle::default());

    // the size of our sprite
    let size = Vec2::new(30.0, 30.0);
    commands
        //  here we add a Sprite. We can add any bundle of our choice; the
        // only required component is a GlobalTransform
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        // Make it a physics body, by adding the RigidBody component
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            // let the size be consistent with our sprite
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        });
}
