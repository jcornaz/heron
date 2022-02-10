# Heron

[![License](https://img.shields.io/github/license/jcornaz/heron)](https://github.com/jcornaz/heron/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/heron)](https://crates.io/crates/heron)
[![Docs](https://docs.rs/heron/badge.svg)](https://docs.rs/heron)
[![dependency status](https://deps.rs/repo/github/jcornaz/heron/status.svg)](https://deps.rs/repo/github/jcornaz/heron)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![Build](https://img.shields.io/github/workflow/status/jcornaz/heron/Build)](https://github.com/jcornaz/heron/actions?query=workflow%3ABuild+branch%3Amain)
[![Zenhub](https://img.shields.io/badge/workspace-zenhub-%236061be)](https://app.zenhub.com/workspaces/heron-600478067304b1000e27f4c4/board)

An ergonomic physics API for 2d and 3d [bevy] games. (powered by [rapier])

## What it looks like

```rust,no_run
use bevy::prelude::*;
use heron::prelude::*;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(PhysicsPlugin::default()) // Add the plugin
    .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0))) // Optionally define gravity
    .add_startup_system(spawn.system())
    .run();
}

fn spawn(mut commands: Commands) {
    commands

        // Spawn any bundle of your choice. Only make sure there is a `GlobalTransform`
        .spawn_bundle(SpriteBundle::default())

        // Make it a rigid body
        .insert(RigidBody::Dynamic)
        
        // Attach a collision shape
        .insert(CollisionShape::Sphere { radius: 10.0 })
        
        // Optionally add other useful components...
        .insert(Velocity::from_linear(Vec3::X * 2.0))
        .insert(Acceleration::from_linear(Vec3::X * 1.0))
        .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
        .insert(RotationConstraints::lock())
        .insert(CollisionLayers::none().with_group(Layer::Player).with_mask(Layer::World));
}

// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    Enemies,
}
```

## Installation


**For a 2d game:**
<!--- x-release-please-start-version --->
```toml
heron = { version = "1.1.0", features = ["2d"] }
```
<!--- x-release-please-end-version --->


**For a 3d game:**
<!--- x-release-please-start-version --->
```toml
heron = { version = "1.1.0", features = ["3d"] }
```
<!--- x-release-please-end-version --->


## Supported Bevy Versions

| bevy | heron      |
|------|------------|
| 0.6  | 1          |
| 0.5  | 0.4 - 0.13 |
| 0.4  | 0.1 - 0.3  |

## Design principles

* Use [bevy] types, resources and components when possible (`Vec3`, `Quat`, `Transform`, `Events`, etc.)
* Provide a single API that works for both 2d and 3d. (Like bevy does)
* Data oriented. Using this library should feel like its a part of [bevy].
* Avoid asking the user to lookup in resources via *handles*. Data should be accessible and modifiable directly in components.
* Hide the actual physics engine. This is an implementation detail the user shouldn't have to worry about.
    * But, allow advanced users to access the underlying [rapier] resources, so the user is never blocked by a missing
      element in the API of heron.


## Feature flags

One must choose to use either `2d` or `3d`. If neither of the two features is enabled, the `PhysicsPlugin` won't be available.


* `3d` Enable simulation on the 3 axes `x`, `y`, and `z`.
* `2d` Enable simulation only on the first 2 axes `x` and `y`.
* `debug-2d` Render 2d collision shapes.
* `enhanced-determinism` Enable rapier's [enhanced-determinism](https://rapier.rs/docs/user_guides/rust/determinism)


## How does this project compare to bevy_rapier?

[bevy_rapier] plugin is an excellent option and should definitely be considered.

Here are some key differences between the two projects:

* `heron` tries to provide a smaller, simpler API that is easier to use. `bevy_rapier` is more complete and powerful, but a bit more complex.
* `heron` mostly hides the underlying physics engine, so you don't have to use [rapier] directly nor [nalgebra]. `bevy_rapier` asks the user to deal directly with `rapier` and `nalgebra`.
* `heron` is focused on games only. `bevy_rapier` targets all kind of physics simulation applications (incl. games).
* `bevy_rapier` is actively maintained by [dimforge], the developer of `rapier`. `heron` is also active, but cannot evolve as fast as `bevy_rapier` can. 


`heron` is probably more suited for simple games and game-jams, where the ease of learn/use is especially valuable and where the lack of advanced feature isn't problematic.

`bevy_rapier` is probably more suited for bigger/complex games and other types of physics simulations, where it may be better to learn/use a more exhaustive/complex API. 


## Contribute / Contact

You can open issues/discussions here or you can discuss with me (`Jomag#2675`) in the [bevy discord](https://discord.com/invite/gMUk5Ph)

See [how to contribute](https://github.com/jcornaz/heron/blob/main/CONTRIBUTING.md)


[bevy]: https://bevyengine.org
[rapier]: https://rapier.rs
[bevy_rapier]: https://github.com/dimforge/bevy_rapier
[dimforge]: https://www.dimforge.com
[nalgebra]: https://github.com/dimforge/nalgebra
