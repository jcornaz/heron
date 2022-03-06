use bevy::{prelude::*, utils::HashSet};

use crate::CollisionEvent;

/// Component which will be filled with a list of entities with which the current entity is currently in contact (if present).
#[derive(Component, Default, Reflect)]
pub struct CollidingEntities(pub HashSet<Entity>);

/// Adds entity to [`CollidingEntities`] on starting collision and removes from it when the
/// collision end.
pub(super) fn update_colliding_entities(
    mut collision_events: EventReader<'_, '_, CollisionEvent>,
    mut colliding_entities: Query<'_, '_, &mut CollidingEntities>,
) {
    for event in collision_events.iter() {
        let (entity1, entity2) = event.rigid_body_entities();
        if event.is_started() {
            if let Ok(mut entities) = colliding_entities.get_mut(entity1) {
                entities.0.insert(entity2);
            }
            if let Ok(mut entities) = colliding_entities.get_mut(entity2) {
                entities.0.insert(entity1);
            }
        } else {
            if let Ok(mut entities) = colliding_entities.get_mut(entity1) {
                entities.0.remove(&entity2);
            }
            if let Ok(mut entities) = colliding_entities.get_mut(entity2) {
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
    fn colliding_entities_updates() {
        let mut app = App::new();
        app.add_event::<CollisionEvent>()
            .add_system(update_colliding_entities);

        let entity1 = app.world.spawn().insert(CollidingEntities::default()).id();
        let entity2 = app.world.spawn().insert(CollidingEntities::default()).id();

        let collision_data1 =
            CollisionData::new(entity1, Entity::from_raw(0), CollisionLayers::default(), []);
        let collision_data2 =
            CollisionData::new(entity2, Entity::from_raw(0), CollisionLayers::default(), []);
        let mut collision_events = app
            .world
            .get_resource_mut::<Events<CollisionEvent>>()
            .unwrap();
        collision_events.send(CollisionEvent::Started(collision_data1.clone(), collision_data2.clone()));

        app.update();

        let colliding_entities1 = app
            .world
            .entity(entity1)
            .get::<CollidingEntities>()
            .unwrap();
        assert_eq!(
            colliding_entities1.0.len(),
            1,
            "There should be one colliding entity"
        );
        assert_eq!(
            colliding_entities1.0.iter().next().unwrap(),
            &entity2,
            "Colliding entity should be equal to second entity"
        );

        let colliding_entities2 = app
            .world
            .entity(entity2)
            .get::<CollidingEntities>()
            .unwrap();
        assert_eq!(
            colliding_entities2.0.len(),
            1,
            "There should be one colliding entity"
        );
        assert_eq!(
            colliding_entities2.0.iter().next().unwrap(),
            &entity1,
            "Colliding entity should be equal to second entity"
        );

        let mut collision_events = app
            .world
            .get_resource_mut::<Events<CollisionEvent>>()
            .unwrap();
        collision_events.send(CollisionEvent::Stopped(collision_data1, collision_data2));

        app.update();

        let colliding_entities1 = app
            .world
            .entity(entity1)
            .get::<CollidingEntities>()
            .unwrap();
        assert_eq!(
            colliding_entities1.0.len(),
            0,
            "Colliding entity should be removed from the list when the collision ends"
        );

        let colliding_entities2 = app
            .world
            .entity(entity2)
            .get::<CollidingEntities>()
            .unwrap();
        assert_eq!(
            colliding_entities2.0.len(),
            0,
            "Colliding entity should be removed from the list when the collision ends"
        );

    }
}
