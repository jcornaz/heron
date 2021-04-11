# Quickstart

Here is a basic example of using Heron with 2d graphics. It draws a green box
that falls down due to gravity:

```rust,no_run
use bevy::prelude::*;
use heron::prelude::*;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the Heron plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0))) // Define gravity
        .add_startup_system(spawn.system())
        .run();
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Ensure we can see things
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // the size of our sprite
    let size = Vec2::new(30.0, 30.0);
    commands
        //  here we add a Sprite. We can add any bundle of our choice; the
        // only required component is a GlobalTransform
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        // Make it a physics body, by attaching a collision shape
        .insert(Body::Cuboid {
            // let the size be consistent with our sprite
            half_extends: size.extend(0.0) / 2.0,
        });
}
```

If you create a new project using `cargo init` you can replace `main.rs`
with this code. You should also add this to your projects's `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.5"} 
heron = { version = "0.4.0", default-features = false, features = ["2d"] }
```

Heron defaults to 3d. To make sure we run it in 2d mode we have to configure it
in `Cargo.toml`; we need to turn off the default features and enable `2d`.

If you then run `cargo run` you should see the green box fall.

## Explanation

We create a normal `Bevy` app. To enable Heron we must add the `PhysicsPlugin`.
Optionally you can add a `Gravity` resource, as we do here.

We can spawn our physics entities in the startup system as we do here.

We also need to set up the camera bundle to 2d so that we can see something.

We then spawn a `SpriteBundle` in the normal Bevy style; here we generate a
very basic green `box` sprite and place it on `x` and `y` coordinates. Note
that we have to use `Vec3` even though we are in 2d space. In 2d we simply set
the `z` coordinate to zero always.

To make it work with the physics engine, we must add a Heron `Body` component.
In this case we add a cuboid (rectangular in 2d) collision shape.

And that's all there is to it! Heron, using the Rapier physics engine, makes
your sprite behave according to physics!

You can do a lot more with Heron but this should get you started!
