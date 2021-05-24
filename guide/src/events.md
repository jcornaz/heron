# Collision Events

When two entities collide, a `CollisionEvent` is written to the `Events<CollisionEvent>` resource.

The events contains two `CollisionData` that contains each:

* The bevy `Entity` containing the concerned `RigidBody`
* The bevy `Entity` containing the concerned `CollisionShape`
* The `CollisionLayers` associated with the concerned `CollisionShape`

You may subscribe to the events like with any other event resource in bevy:

```rust,no_run,noplayground
{{#include ../../examples/events.rs:log-collisions}}
```

Combining `CollisionData` and `CollisionLayers` can make easy to plugin gameplay logic.

As an example, let's say we have a player inside the `Layer::Player` collision group, and an enemy inside the `Layer::Enemy` collision group. (see: [layers](layers.md) for more informations on layer groups/masks)

If we want to kill the enemy when the player collides with it we can write something like this:

```rust,no_run,noplayground
{{#include ../../examples/events.rs:kill-enemy}}
```
