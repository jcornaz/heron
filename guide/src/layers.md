# Layers

In a video game, it is quite rare that every type of entity collides with everything.
A typical example is the enemy hurtbox that only needs to collide with the player hitbox. But not with other enemies hitboxes, nor the ground, etc.

In heron, this is achieved with *layers*

## 1. Define a layer enum

This enum list and give names to the physics layer of your game.

```rust,no_run,noplayground
{{#include ../../examples/layers.rs:layer-enum}}
```

There can be a maximum of 32 layers.

## 2. Add the CollisionLayers component to the concerned entities.

That  component contains two collections of layers: "groups" and "masks".

You can think of the "group" as: "in what group this entity is".

And you can think of the "mask" as: "what other groups the entity can interact with".

Two entities A and B will interact if:
 * There is a layer in the groups of A that is also in the masks of B
 * There is a layer in the groups of B that is also in the masks of A

To build an instance of this component, start with either `CollisionLayers::new()`, `CollisionLayers::all()` or `CollisionLayers::none()`,
and then add or remove layers by calling  `with_group`/`without_group` and `with_mask`/`without_mask`.

### Mark an entity to be in the "world" group and collide with the "player"
```rust,no_run,noplayground
{{#include ../../examples/layers.rs:layer-component-world}}
```

### Mark an entity to be in the "entity" group and collide with the "world":
```rust,no_run,noplayground
{{#include ../../examples/layers.rs:layer-component-player}}
```

By default, if the `CollisionLayers` is absent, the entity will collide with everything. 
