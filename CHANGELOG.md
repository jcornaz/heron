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



[Unreleased]: ../../compare/v0.5.1...HEAD
[0.5.1]: ../../compare/v0.5.0...v0.5.1
[0.5.0]: ../../compare/v0.4.0...v0.5.0
[0.4.0]: ../../compare/v0.3.0...v0.4.0
[0.3.0]: ../../compare/v0.2.0...v0.3.0
[0.2.0]: ../../compare/v0.1.1...v0.2.0
[0.1.1]: ../../compare/v0.1.0...v0.1.1
[0.1.0]: ../../compare/v0.1.0-alpha.1...v0.1.0
[0.1.0-alpha.1]: ../../compare/...v0.1.0-alpha.1
