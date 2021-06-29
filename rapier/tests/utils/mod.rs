#[allow(unused_imports)]
#[cfg(dim2)]
pub use heron_rapier::rapier2d::{
    dynamics::{IntegrationParameters, JointSet, MassProperties, RigidBodyHandle, RigidBodySet},
    geometry::{ColliderHandle, ColliderSet},
    math::Vector,
};
#[cfg(dim3)]
pub use heron_rapier::rapier3d::{
    dynamics::{IntegrationParameters, JointSet, MassProperties, RigidBodyHandle, RigidBodySet},
    geometry::{ColliderHandle, ColliderSet},
    math::Vector,
};
