# Heron

[![License](https://img.shields.io/github/license/jcornaz/heron)](https://github.com/jcornaz/heron/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/heron)](https://crates.io/crates/heron)
[![Docs](https://docs.rs/heron/badge.svg)](https://docs.rs/heron)
[![dependency status](https://deps.rs/repo/github/jcornaz/heron/status.svg)](https://deps.rs/repo/github/jcornaz/heron)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![Build](https://img.shields.io/github/workflow/status/jcornaz/heron/Build)](https://github.com/jcornaz/heron/actions?query=workflow%3ABuild+branch%3Amain)
[![Zenhub](https://img.shields.io/badge/workspace-zenhub-%236061be)](https://app.zenhub.com/workspaces/heron-600478067304b1000e27f4c4/board)

An ergonomic physics API for 2d and 3d [bevy] games. (powered by [rapier])

## How it looks like

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
        .spawn(SpriteBundle::default())

        // Make it a physics body, by attaching a collision shape
        .with(Body::Sphere { radius: 10.0 })

        // Optionally define a type (if absent, the body will be *dynamic*)
        .with(BodyType::Kinematic)
        
        // Optionally define the velocity (works only with dynamic and kinematic bodies)
        .with(Velocity::from(Vec2::unit_x() * 2.0));
}
```

## Installation


**For a 3d game:**
```toml
bevy = "^0.4.0"
heron = "0.3.0"
```

**For a 2d game:**
```toml
bevy = "^0.4.0"
heron = { version = "0.3.0" default-features = false, features = ["2d"] }
```


## Design principles

* Use [bevy] types, resources and components when possible (`Vec3`, `Quat`, `Transform`, `Events`, etc.)
* Provide a single API that works for both 2d and 3d. (Like bevy does)
* Data oriented. Using this library should look like it is part of [bevy].
* Avoid asking the user to lookup in resources via *handles*. Data should be accessible and modifiable directly in components.
* Hide the actual physics engine. This is an implementation detail the user shouldn't have to care about.
    * But, allow advanced users to access the underlying [rapier] resources, so a user is never blocked by a missing
      element in the API of heron.


## Feature flags

One must choose to use either `2d` or `3d` (but not both). If none of theses two features is enabled, the `PhysicsPlugin` won't be available.

### Enabled by Default

* `3d` Enable simulation on the 3 axes `x`, `y`, and `z`. Incompatible with the feature `2d`.

### Optional

* `2d` Enable simulation only on the first 2 axes `x` and `y`. Incompatible with the feature `3d`, therefore require to disable the default features.
* `debug` Render collision shapes. Works only in 2d, support for 3d will be added later.


## Motivation

I think [rapier] is very powerful as a physics engine. But using it directly or via [bevy_rapier] in a [bevy] game is
not ergonomic enough for my taste.

Ideally I would like to have the *power* of [rapier] accessible behind an API focused on [bevy] *games*.


[bevy]: https://bevyengine.org

[rapier]: https://rapier.rs

[bevy_rapier]: https://github.com/dimforge/bevy_rapier


## Contribute / Contact

You can open issues/discussions here or you can discuss with me (`Jomag#2675`) in the [bevy discord](https://discord.com/invite/gMUk5Ph)

See [how to contribute](https://github.com/jcornaz/heron/blob/main/CONTRIBUTING.md)
