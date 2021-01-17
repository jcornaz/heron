# Changelog

All notable changes to this project are documented in this file.

The format is inspired from [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

### PhysicsPlugin plugin

Add the `PhysicsPlugin` to setup collision detection and physics simulation. It also registers rapier's `RigidBodySet`
, `ColliderSet`, `JointSet`, `IntegrationParameters`, `BroadPhase` and `NarrowPhase` in the resources.

### Gravity resource

The resource `Gravity` defines the world's gravity. Gravity is disabled by default. You may override or mutate
the `Gravity` resource to change the world's gravity.

### Body component

A `Body` component can be added to make the entity a *dynamic* rigid body with the given shape.

The position of the body is defined by the bevy `GlobalTransform`. Updating the `GlobalTransform`, will cause teleport
of the body ignoring physics rules.

Every frame the `Transform` will be updated to reflect the body position in the world.

Heron will automatically handle removal of the body (when the component is removed or when the entity is despawned)

Right now, only sphere are supported. More shape will be added in the future. Support for static and kinematic bodies
will be added later too.


[Unreleased]: ../../compare/...HEAD
