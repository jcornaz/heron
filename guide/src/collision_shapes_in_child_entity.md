# Collision shapes in child entity

The `CollisionShape` component doesn't have to be on the same entity as the `RigidBody`. It can also be on a direct
child entity of the `RigidBody` component.

This can be useful to position/rotate the shape relatively to its parent rigid body.

It is also possible to add multiple collision shape for a single rigid body. When doing so the `SensorShape` component
can be added to create a mix of physics and sensor shapes.

Example:

```rust,no_run,noplayground
{{#include ../../examples/collision_shapes_in_child_entity.rs:add-child-shape}}
```
