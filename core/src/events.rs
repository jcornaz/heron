use bevy::{ecs::entity::Entity, math::Vec3};
use smallvec::SmallVec;

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
///             CollisionEvent::Started(data1, data2) => {
///                 println!("Entity {:?} and {:?} started to collide", data1.rigid_body_entity(), data2.rigid_body_entity())
///             }
///             CollisionEvent::Stopped(data1, data2) => {
///                 println!("Entity {:?} and {:?} stopped to collide", data1.rigid_body_entity(), data2.rigid_body_entity())
///             }
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub enum CollisionEvent {
    /// The two entities started to collide
    Started(CollisionData, CollisionData),

    /// The two entities no longer collide
    Stopped(CollisionData, CollisionData),
}

/// Collision data concerning one of the two entity that collided
#[derive(Debug, Clone)]
pub struct CollisionData {
    rigid_body_entity: Entity,
    collision_shape_entity: Entity,
    collision_layers: CollisionLayers,
    normals: SmallVec<[Vec3; 1]>,
}

impl From<CollisionEvent> for (CollisionData, CollisionData) {
    fn from(event: CollisionEvent) -> Self {
        event.data()
    }
}

impl CollisionEvent {
    /// Returns true if the event represent the "start" of a collision
    #[must_use]
    pub fn is_started(&self) -> bool {
        matches!(self, CollisionEvent::Started(_, _))
    }

    /// Returns true if the event represent the "end" of a collision
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        matches!(self, CollisionEvent::Stopped(_, _))
    }

    /// Returns the data for the two entities that collided
    #[must_use]
    pub fn data(self) -> (CollisionData, CollisionData) {
        match self {
            CollisionEvent::Started(d1, d2) | CollisionEvent::Stopped(d1, d2) => (d1, d2),
        }
    }

    /// Returns the entities containing the [`CollisionShape`](crate::RigidBody) involved in the collision
    #[must_use]
    pub fn collision_shape_entities(&self) -> (Entity, Entity) {
        match self {
            CollisionEvent::Started(d1, d2) | CollisionEvent::Stopped(d1, d2) => {
                (d1.collision_shape_entity, d2.collision_shape_entity)
            }
        }
    }

    /// Returns the entities containing the [`RigidBody`](crate::RigidBody) involved in the collision
    #[must_use]
    pub fn rigid_body_entities(&self) -> (Entity, Entity) {
        match self {
            CollisionEvent::Started(d1, d2) | CollisionEvent::Stopped(d1, d2) => {
                (d1.rigid_body_entity, d2.rigid_body_entity)
            }
        }
    }

    /// Returns the two [`CollisionLayers`] involved in the collision
    #[must_use]
    pub fn collision_layers(&self) -> (CollisionLayers, CollisionLayers) {
        match self {
            CollisionEvent::Started(d1, d2) | CollisionEvent::Stopped(d1, d2) => {
                (d1.collision_layers, d2.collision_layers)
            }
        }
    }
}

impl CollisionData {
    #[must_use]
    #[allow(missing_docs)]
    pub fn new(
        rigid_body_entity: Entity,
        collision_shape_entity: Entity,
        collision_layers: CollisionLayers,
        normals: impl IntoIterator<Item = Vec3>,
    ) -> Self {
        Self {
            rigid_body_entity,
            collision_shape_entity,
            collision_layers,
            normals: normals.into_iter().collect(),
        }
    }

    /// Returns the entity containing the [`RigidBody`](crate::RigidBody)
    #[must_use]
    pub fn rigid_body_entity(&self) -> Entity {
        self.rigid_body_entity
    }

    /// Returns the entity containing the [`CollisionShape`](crate::CollisionShape)
    #[must_use]
    pub fn collision_shape_entity(&self) -> Entity {
        self.collision_shape_entity
    }

    /// Returns the [`CollisionLayers`] of the collision shape entity
    #[must_use]
    pub fn collision_layers(&self) -> CollisionLayers {
        self.collision_layers
    }

    /// Returns the normal vector pointing toward this entity and away from the other entity
    #[must_use]
    pub fn normals(&self) -> &[Vec3] {
        &self.normals
    }
}
