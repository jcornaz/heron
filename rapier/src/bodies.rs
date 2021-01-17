use bevy_ecs::{
    Changed, Commands, IntoChainSystem, IntoSystem, Mutated, Query, ResMut, System, With, Without,
};

use crate::rapier::dynamics::RigidBodySet;
use crate::rapier::geometry::ColliderSet;
use crate::BodyHandle;
use heron_core::Body;

pub(crate) fn maintenance() -> impl System<In = (), Out = ()> {
    delete_bodies
        .system()
        .chain(update_bodies.system())
        .chain(create_bodies.system())
}

pub fn create_bodies(
    commands: &mut Commands,
    mut body_set: ResMut<RigidBodySet>,
    mut collider_set: ResMut<ColliderSet>,
    query: Query<(&Body,), Without<BodyHandle>>,
) {
    todo!()
}

pub fn update_bodies(
    mut body_set: ResMut<RigidBodySet>,
    mut collider_set: ResMut<ColliderSet>,
    query: Query<&Body, (Mutated<Body>, With<BodyHandle>)>,
) {
    todo!()
}

pub fn delete_bodies() {
    todo!()
}
