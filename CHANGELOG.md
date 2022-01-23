# Changelog

All notable changes to this project are documented in this file.

The format is inspired from [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

### Bug fixes

* **2d-debug**: crash when despawning recursivly an entity (#178) 


## [1.0.1-rc.1] - 2022-01-09

### Breaking changes

* Require bevy version 0.6
* Require rust 1.57
* Remove debug-3d feature (This will hopefully be reintroduced when upstream dependencies will be compatible with bevy 0.6)

## [0.13.0] - 2021-12-31

### Breaking changes

* The `CollisionShape` is now marked `#[non_exhaustive]`
* The `AppBuilderExt` trait and the `add_physics_system` is removed
  * This was no longer necessary since #109

### Added

* Cone collision shape (#158)
* Cylinder collision shape (#159)
* Custom collision shape (#160)
* Damping component (#164)

### Fixed

* Despawn not working if simulation is paused


## [0.12.1] - 2021-10-24

## Fixed

* `KinematicVelocityBased` bodies not being moved by velocity (#148).
* Collision events not being fired for kinematic bodies (#142).


## [0.12.0] - 2021-10-24

### Breaking changes

* The `DebugColor` resource no longer implement `From<Color>`. That's because it now defines multiples colors, not one.
* The required version of `rapier` is now `^0.11.1`
* The minimum supported version of rust is now `1.56.0`


### Added

* Debug renderer for 3d, behind the `debug-3d` feature flag (#151).  


### Changed

* The collision shapes now have different colors based on the type of body (static, dynamic, kinematic, sensor).


## 0.11.1 - 2021-08-23

### Bug fixes

* Too broad scaling of all shapes with debug mode (#138)



## 0.11.0 - 2021-07-19

### Breaking changes

* `CollisionShape::Cuboid` and `CollisionShape::ConvexHull` now have `rounded` member (that can be `None`)


### Features

* `PhysicsWorld` system parameter that can be used for ray and shape casting (#129)


## 0.10.1 - 2021-07-19 [YANKED]

### YANKED

This release is yanked because breaking changes have accidentally landed in this minor semver bump.

It has been re-released under the version number `0.11.0` to reflect the presence of breaking changes.

### Breaking changes

* `CollisionShape::Cuboid` and `CollisionShape::ConvexHull` now have `rounded` member (that can be `None`)


### Features

* `PhysicsWorld` system parameter that can be used for ray and shape casting (#129)



## 0.10.0 - 2021-07-12

### Breaking changes

* `PhysicsSteps::duration` now returns a `PhysicsDuration` instead of `Duration`


### Features

* Variable timestep mode for PhysicsSteps (#123)



## 0.9.1 - 2021-06-30

### Bug fixes

* Empty documentation on docs.rs



## 0.9.0 - 2021-06-29

### Breaking changes

* The cargo feature `debug` has been replaced by `debug-2d`. (thanks @zicklag)
* No cargo feature is enabled by default. One must explictly choose between 2d or 3d. (Thanks @zicklag)
* The `heron_rapier::rapier` module has been replaced by `heron_rapier::rapier2` in 2d and `heron_rapier::rapier3d` in 3d.
* Required version of rapier is now ^0.9.2. (Thanks @zicklag)
* RigidBody::Static is replaced by `RigidBody::KinematicPositionBased` and `RigidBody::KinematicVelocityBased`. (Thanks @zicklag)
* The `Layer` trait now works with `u32` instead of `u16`, increasing the maximum number of layers up to 32. (Thanks @zicklag)


### Features

#### 2d

* Preserve Z translation (#120, thanks @zicklag)


### Bug fixes

* Impossiblity to disable render feature in 2d (#118, thanks @zicklag)
* Compilation errors for 3d users (#125)



## 0.8.0 - 2021-06-07

### Breaking changes

* There is a new variant `HeightField` to the `CollisionShape` enum


### Features

* HeightField collision shape (#102, thanks @elegaanz)



## 0.7.0 - 2021-05-31

### Breaking changes

* The `CollisionEvent` now contains a pair of `CollisionData` instead of a pair of `Entity`
* The physics step rate is not defined on the plugin type anymore. It is replaced by a `PhysicsSteps` resource.
* The `IntegrationParameters` can no longer be defined when installing the plugin. The `IntegrationParameters` should should be inserted/mutated instead.
* The `Velocity` and `Transform` are now updated during the `CoreStage::PostUpdate`. If a user-system need to run *after* the physics update, it should explicitly declares that dependency using the `PhysicsSystem` labels.


### Features

* More data in collision events (#103)
* `SensorShape` component to mark individual collision shape as sensor (#104)
* Runs physics step at most once per frame (#109)



## 0.6.0 - 2021-05-24

### Breaking changes

* The `Body` component is renamed to `CollisionShape`
* The `BodyType` component is renamed to `RigidBody` and is now mandatory. (The rigid bodies are no longer automatically "dynamic" when the component is not found)
* The `BodyHandle` component is removed. The rapier's `RigidBodyHandle` and `ColliderHandle` are now used instead.


### Features

* Allow to define (multiple) collision shapes in the child entity of a rigid body (#97)
* Collision layers (#101)


### Bug fixes

* Frame delay after a spawn/update of the entity transform (#92)
* The acceleration component is now registered for bevy reflection



## [0.5.1] - 2021-05-01

### PhysicsTime resource

The `PhysicsTime` resource can be used to change the time-scale of the physics engine.

Thanks @yaymalaga


## [0.5.0] - 2021-04-18

### ⚠ Dependency requirements updated (breaking)

The required version of rapier2d is bumped to ^0.8.0

### ⚠ Support defining friction

A new public field `friction` has been added in `PhysicMaterial`.
As the name suggest, it allows to define the friction that should be applied when two rigid bodies are in contact.

Thanks @yaymalaga


## [0.4.0] - 2021-04-17

### ⚠ Dependency requirements updated (breaking)

The required version of bevy is bumped to ^0.5.0
The required version of rapier2d is bumped to ^0.7.2
The required version of rapier3d is bumped to ^0.8.0

### Feature flags of public dependencies

Heron no longer enables any feature flag for rapier. That way the end-user can freely choose the rapier feature flags.


## [0.3.0] - 2021-03-21

### ⚠ ConvexHull collision shape

There is a new variant in the `Body` enum: `ConvexHull`. It takes a list of points, and the body shape will be the
smallest possible convex shape that includes all the given points.

Thanks @faassen

### Acceleration component

A new `Acceleration` component make possible to apply and linear and angular accelerations.
It is by extension also possible to apply a force if you know the mass: `acceleration.linear = force / mass`.

Thanks @ryo33

### RotationConstraints component

A new `RotationConstraints` component make possible to prevent rotation around the given axes.

### Others

* The opacity has been increased for the default color of debug shapes.


## [0.2.0] - 2021-03-07

### ⚠ Physics systems

The physics step run at a fixed rate (60 updates per second by default). Therefore, it is not in sync with the frame update (that runs as many times per second as possible).

But a user may want to (and sometime have to) run system synchronously with the physics step.

This is why two stages are now public:
* `stage::ROOT`: the root **schedule** stage that contains the physics step and run at a fixed rate (60 updates per second by default)
* `stage::UPDATE`: a **child** (parallel) system stage that runs before each physics step

But most of the time, users shouldn't have to use the stage directly,
because an `add_physics_system` extension function on `AppBuilder` is provided and can be used like `add_system`, except
systems added with `add_physics_system` will run during the physics update.

**This is a breaking change:** Updating the transforms/velocities or any other physics component of rigid bodies **must** be done in the physics update stage.
Make sure to add theses systems using the new `add_physics_system` extension function on `AppBuilder`.


### ⚠ New `PhysicMaterial` component that replaces `Restitution` (breaking)

There is now a `PhysicMaterial` component which can be used to define both the restitution (how bouncy) and density (how heavy) the material is.

In the future it will be extended to define more physics properties, like the friction.

Since the restitution is now defined in `PhysicMaterial`, the `Restitution` component has been removed.

### ⚠ Kinematic bodies

There is a new variant to `BodyType`: `Kinematic`. That makes possible to create "kinematic" bodies.
A kinematic body is controlled programmatically (usually by updating the transform) and affect the other bodies normally, 
but is not affected by them.


### ⚠ Dependency requirements updated (breaking)

The required version of rapier is bumped to ^0.6.1

### All components are registered for reflection

All components now implement `Default` and `Reflect` and are registered to bevy reflect system.
That should make be possible to use heron components in serialized scene for hot-reloading.


### Public constructor to `BodyHandle`

`BodyHandle` now has a public constructor. Advanced users may create rigid bodies and colliders using directly the rapier API (adding them to the `RigidBodySet` and `ColliderSet` resources), and then add a `BodyHandle` component to the entity so that heron's will handle velocity and update the bevy transforms.

Tanks @MGlolenstine


## [0.1.1] - 2021-02-16

### ⚠ Fix incorrect internal version requirements

A problem happened during the release of `0.1.0`, and some crates (incl. the root crate `heron`)
were requiring invalid version of the other heron crates.


## [0.1.0] - 2021-02-15 [YANKED]

### ⚠ Dependency requirement updated (breaking)

The required rapier version is now >= 0.5.0 < 0.6.0.


### ⚠ New collision shapes (breaking)

The variants `Body::Capsule` and `Body::Cuboid` have been added, allowing creating respectively capsule and cuboid
collision shapes


### New `BodyType` component

By default, the rigid-bodies are *dynamic*. A `BodyType` can be attached to make it:
* Static (with `BodyType::Static`) so that it doesn't move.
* Sensor (with `BodyType::Sensor`) so it doesn't move and doesn't affect other bodies. (Only useful for listening to collision events)


### Fixes

* Misplaced debug render at startup
* Incorrect angular velocity in 2d

## [0.1.0-alpha.1] - 2021-01-30

### Features flags

* `3d` (enabled by default) Enable simulation on the 3 axes `x`, `y`, and `z`. Incompatible with the feature `2d`.
* `2d` Enable simulation only on the first 2 axes `x` and `y`. Incompatible with the feature `3d`, therefore require to
  disable the default features.
* `debug` Render collision shapes. Works only in 2d for now, support for 3d will be added later.

Important: Either `2d` or `3d` (but not both) must be enabled. If none or both of theses two features are enabled,
the `PhysicsPlugin` won't be available.

### PhysicsPlugin plugin

Add the `PhysicsPlugin` to setup collision detection and physics simulation. It also registers rapier's `RigidBodySet`
, `ColliderSet`, `JointSet`, `IntegrationParameters`, `BroadPhase` and `NarrowPhase` in the resources.

### Gravity resource

The resource `Gravity` defines the world's gravity. Gravity is disabled by default. You may override or mutate
the `Gravity` resource to change the world's gravity.

### Body component

A `Body` component can be added to make the entity a *dynamic* rigid body with the given shape.

The position of the body is defined by the bevy `GlobalTransform` component. Updating the `GlobalTransform`, will
teleport the body ignoring physics rules.

Every frame the `Transform` will be updated to reflect the body position in the world.

Heron will automatically handle replacement and removal of the body (when the component mutated/removed or when the
entity is despawned)

At the moment, only spheres are supported. More shape will be added in the future. Support for static and kinematic
bodies will be added later too.

### Velocity component

Add the `Velocity` component to an entity to define/update or read the velocity of a dynamic body.

The entity, must also have a `Body` component and a `GlobalTransform`.

### Restitution component

The `Restitution` component can be added to define the restitution coefficient of a body.

### CollisionEvent event

One can read from `Events<CollisionEvent>` to be notified when collisions start and stop.



[Unreleased]: ../../compare/v1.0.1-rc.1...HEAD
[1.0.1-rc.1]: ../../compare/v0.13.0...v1.0.1-rc.1
[0.13.0]: ../../compare/v0.12.1...v0.13.0
[0.12.1]: ../../compare/v0.12.0...v0.12.1
[0.12.0]: ../../compare/v0.11.1...v0.12.0
[0.5.1]: ../../compare/v0.5.0...v0.5.1
[0.5.0]: ../../compare/v0.4.0...v0.5.0
[0.4.0]: ../../compare/v0.3.0...v0.4.0
[0.3.0]: ../../compare/v0.2.0...v0.3.0
[0.2.0]: ../../compare/v0.1.1...v0.2.0
[0.1.1]: ../../compare/v0.1.0...v0.1.1
[0.1.0]: ../../compare/v0.1.0-alpha.1...v0.1.0
[0.1.0-alpha.1]: ../../compare/...v0.1.0-alpha.1
