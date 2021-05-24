use bevy::ecs::entity::Entity;

use crate::CollisionLayers;

/// An event fired when the collision state between two entities changed
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn detect_collisions(mut events: EventReader<CollisionEvent>) {
///     for event in events.iter() {
///         match event {
///             CollisionEvent::Started(entity1, entity2) => println!("Entity {:?} and {:?} started to collide", entity1, entity2),
///             CollisionEvent::Stopped(entity1, entity2) => println!("Entity {:?} and {:?} stopped to collide", entity1, entity2),
///         }
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CollisionEvent {
    /// The two entities started to collide
    Started(CollisionData, CollisionData),

    /// The two entities no longer collide
    Stopped(CollisionData, CollisionData),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CollisionData {
    rigid_body_entity: Entity,
    collision_shape_entity: Entity,
    collision_layers: CollisionLayers,
}

impl From<CollisionEvent> for (CollisionData, CollisionData) {
    fn from(event: CollisionEvent) -> Self {
        event.data()
    }
}

impl CollisionEvent {
    pub fn data(self) -> (CollisionData, CollisionData) {
        match self {
            CollisionEvent::Started(d1, d2) => (d1, d2),
            CollisionEvent::Stopped(d1, d2) => (d1, d2),
        }
    }

    pub fn collision_shape_entities(&self) -> (Entity, Entity) {
        match self {
            CollisionEvent::Started(d1, d2) => {
                (d1.collision_shape_entity, d2.collision_shape_entity)
            }
            CollisionEvent::Stopped(d1, d2) => {
                (d1.collision_shape_entity, d2.collision_shape_entity)
            }
        }
    }

    pub fn rigid_body_entities(&self) -> (Entity, Entity) {
        match self {
            CollisionEvent::Started(d1, d2) => (d1.rigid_body_entity, d2.rigid_body_entity),
            CollisionEvent::Stopped(d1, d2) => (d1.rigid_body_entity, d2.rigid_body_entity),
        }
    }
}

impl CollisionData {
    pub fn new(
        rigid_body_entity: Entity,
        collision_shape_entity: Entity,
        collision_layers: CollisionLayers,
    ) -> Self {
        Self {
            rigid_body_entity,
            collision_shape_entity,
            collision_layers,
        }
    }

    pub fn rigid_body_entity(&self) -> Entity {
        self.rigid_body_entity
    }

    pub fn collision_shape_entity(&self) -> Entity {
        self.collision_shape_entity
    }

    pub fn collision_layers(&self) -> CollisionLayers {
        self.collision_layers
    }
}
