# Collision shapes in child entity

The `CollisionShape` component doesn't have to be on the same entity than the `RigidBody` component. It can also be on a
direct child entity of the `RigidBody` component.

Example:

```rust,no_run,noplayground
{{#include ../../examples/collision_shapes_in_child_entity.rs:add-child-shape}}
```

This makes possible to:

* define the position and rotation of the collision shape relative to its parent rigid body.
* define multiple collision shapes for a single rigid body.
