#[allow(unused_imports)]
#[cfg(dim2)]
pub use heron_rapier::rapier2d::{
    dynamics::{IntegrationParameters, JointSet, MassProperties, RigidBodyDamping, RigidBodySet},
    geometry::ColliderSet,
    math::Vector,
};
#[cfg(dim3)]
pub use heron_rapier::rapier3d::{
    dynamics::{IntegrationParameters, JointSet, MassProperties, RigidBodyDamping, RigidBodySet},
    geometry::ColliderSet,
    math::Vector,
};

use heron_rapier::{ColliderHandle, RigidBodyHandle};
