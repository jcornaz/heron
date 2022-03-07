use bevy::{prelude::*, utils::HashSet};

use crate::CollisionEvent;

/// Component which will be filled (if present) with a list of entities with which the current entity is currently in contact.
#[derive(Component, Default, Reflect)]
pub struct Collisions(HashSet<Entity>);

impl Collisions {
    /// Returns the number of colliding entities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the collisions contains the specified entity.
    #[must_use]
    pub fn contains(&self, entity: &Entity) -> bool {
        self.0.contains(entity)
    }

    /// An iterator visiting all colliding entities in arbitrary order.
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

/// Adds entity to [`CollidingEntities`] on starting collision and removes from it when the
/// collision end.
pub(super) fn update_collisions(
    mut collision_events: EventReader<'_, '_, CollisionEvent>,
    mut collisions: Query<'_, '_, &mut Collisions>,
) {
    for event in collision_events.iter() {
        let (entity1, entity2) = event.rigid_body_entities();
        if event.is_started() {
            if let Ok(mut entities) = collisions.get_mut(entity1) {
                entities.0.insert(entity2);
            }
            if let Ok(mut entities) = collisions.get_mut(entity2) {
                entities.0.insert(entity1);
            }
        } else {
            if let Ok(mut entities) = collisions.get_mut(entity1) {
                entities.0.remove(&entity2);
            }
            if let Ok(mut entities) = collisions.get_mut(entity2) {
                entities.0.remove(&entity1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::Events;

    use crate::{CollisionData, CollisionLayers};

    use super::*;

    #[test]
    fn collisions_update() {
        let mut app = App::new();
        app.add_event::<CollisionEvent>()
            .add_system(update_collisions);

        let entity1 = app.world.spawn().insert(Collisions::default()).id();
        let entity2 = app.world.spawn().insert(Collisions::default()).id();

        let collision_data1 =
            CollisionData::new(entity1, Entity::from_raw(0), CollisionLayers::default(), []);
        let collision_data2 =
            CollisionData::new(entity2, Entity::from_raw(0), CollisionLayers::default(), []);
        let mut collision_events = app
            .world
            .get_resource_mut::<Events<CollisionEvent>>()
            .unwrap();
        collision_events.send(CollisionEvent::Started(
            collision_data1.clone(),
            collision_data2.clone(),
        ));

        app.update();

        let collisions1 = app.world.entity(entity1).get::<Collisions>().unwrap();
        assert_eq!(collisions1.len(), 1, "There should be one colliding entity");
        assert_eq!(
            collisions1.iter().next().unwrap(),
            &entity2,
            "Colliding entity should be equal to second entity"
        );

        let collisions2 = app.world.entity(entity2).get::<Collisions>().unwrap();
        assert_eq!(collisions2.len(), 1, "There should be one colliding entity");
        assert_eq!(
            collisions2.iter().next().unwrap(),
            &entity1,
            "Colliding entity should be equal to second entity"
        );

        let mut collision_events = app
            .world
            .get_resource_mut::<Events<CollisionEvent>>()
            .unwrap();
        collision_events.send(CollisionEvent::Stopped(collision_data1, collision_data2));

        app.update();

        let collisions1 = app.world.entity(entity1).get::<Collisions>().unwrap();
        assert_eq!(
            collisions1.len(),
            0,
            "Colliding entity should be removed from the list when the collision ends"
        );

        let collisions2 = app.world.entity(entity2).get::<Collisions>().unwrap();
        assert_eq!(
            collisions2.len(),
            0,
            "Colliding entity should be removed from the list when the collision ends"
        );
    }
}
