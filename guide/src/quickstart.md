# Quickstart

Here is a basic example of using Heron with 2d graphics. It draws a green box
that falls down due to gravity:

```rust,no_run,noplayground
{{#include ../../examples/quickstart.rs}}
```

If you create a new project using `cargo init` you can replace `main.rs`
with this code. You should also add this to your projects's `Cargo.toml`:

<!--- x-release-please-start-version --->
```toml
heron = { version = "2.2.0", features = ["2d"] }
```
<!--- x-release-please-end-version --->

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

To make it work with the physics engine, we must add a `RigidBody` and `CollisionShape` components.
In this case we make it a dynamic body with a cuboid (rectangular in 2d) collision shape.

And that's all there is to it! Heron, using the Rapier physics engine, makes
your sprite behave according to physics!

You can do a lot more with Heron but this should get you started!
