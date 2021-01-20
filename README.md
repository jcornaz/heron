# Heron

![Build](https://github.com/jcornaz/heron/workflows/Build/badge.svg)

An ergonomic API to physics in [bevy] 2d and 3d games. (powered by [rapier])


## Design principles

* Don't mirror rapier's API. Simplify what can be simplified. Use bevy idoms when possible. Don't expect users to know
  how [rapier] works.
    * [rapier]'s API targets physics simulation for rust, where Heron targets [bevy] *games*. It is "similar", yes, but
      it isn't "the same".
    * When designing the API, only usage in [bevy] *games* matters. How the rapier's API looks like doesn't matter.
* Use [bevy] types, resources and components when possible (`Vec3`, `Quat`, `Transform`, `Events`, etc.)
* Provide a single API that works for both 2d and 3d. (Like bevy does)
* Data oriented. Using this lib should look like if it was part of [bevy].
* Data is accessible and modifiable directly in components. (Use global resouce only for global config)
* Hide the actual physics engine. This is an implementation detail the user shouldn't have to care about.
    * Yet, allow advanced users to access the underlying [`rapier`] resources, so a user is never blocked by a missing
      element in the API of heron.
* Split concerns in multiple small components/resources.
* Require only the actually *necessary* components. For instance `Velocity` only requires a `Transform`, no need to
  create a rigid body to apply velocity.


## Features

One must choose to use either `2d` or `3d` (but not both). If none of theses two features is enabled, the `PhysicsPlugin` won't be available.

### Enabled by Default

* `3d` Enable simulation on the 3 axes `x`, `y`, and `z`. Incompatible with the feature `2d`.

### Optional

* `2d` Enable simulation only on the first 2 axes `x` and `y`. Incompatible with the feature `3d`, therefore require to disable the default features.
* `debug` Render collision shapes. Works only in 2d, support for 3d will be added later.


## Motivation

I think [rapier] is very powerful as a physics engine. But using it directly or via [bevy_rapier] in a [bevy] game is
not ergonomic enough for my taste.

Ideally I would like to have the *power* of [rapier] accessible behind a nice API designed around [bevy] (not
around [rapier] and [nalgebra]).


[bevy]: https://bevyengine.org

[rapier]: https://rapier.rs

[bevy_rapier]: https://github.com/dimforge/bevy_rapier
