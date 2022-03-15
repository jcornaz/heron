use bevy::{ecs::component::Component, reflect::Reflect};

/// Describes a collision layer
///
/// It is recommended to implement it using the derive macro.
#[allow(missing_docs)]
pub trait PhysicsLayer: Sized {
    fn to_bits(&self) -> u32;
    fn all_bits() -> u32;
}

impl<T: PhysicsLayer> PhysicsLayer for &T {
    fn to_bits(&self) -> u32 {
        T::to_bits(self)
    }

    fn all_bits() -> u32 {
        T::all_bits()
    }
}

/// Components that defines the collision layers of the collision shape.
///
/// This component contains two collections of layers: "groups" and "masks".
///
/// Two entities A and B will interact iff:
///  * There is a layer in the groups of A that is also in the masks of B
///  * There is a layer in the groups of B that is also in the masks of A
///
/// An entity without this component is considered has having all layers in its "groups" and
/// "masks", and will interact with everything.
///
/// This component must be on the same entity of a [`CollisionShape`](crate::CollisionShape)
///
/// To build an instance, start with either [`CollisionLayers::new()`], [`CollisionLayers::all_groups()`],
/// [`CollisionLayers::all_masks()`], [`CollisionLayers::all()`] or
/// [`CollisionLayers::none()`], and then add or remove layers by calling
/// `with_group`/`without_group` and `with_mask`/`without_mask`.
///
/// Theses methods take a type that implement [`PhysicsLayer`]. The best option is to create an enum
/// with a `#[derive(PhysicsLayer)]` clause.
///
/// # Example
///
/// ```
/// # use heron_core::*;
/// # use bevy::prelude::*;
/// # enum GameLayer {
/// #   World,
/// #   Player,
/// #   Enemies,
/// # }
/// # impl PhysicsLayer for GameLayer {
/// #     fn to_bits(&self) -> u32 {
/// #         todo!()
/// #     }
/// #     fn all_bits() -> u32 {
/// #         todo!()
/// #     }
/// # }
/// fn spawn(mut commands: Commands) {
///    commands.spawn_bundle(todo!("Spawn a bundle of your choice"))
///         .insert(RigidBody::Dynamic) // <-- Create a rigid body
///         .insert(CollisionShape::Sphere { radius: 10.0 }) // <-- Attach a collision shape
///         .insert(
///
///             // Define the collision layer of this *collision shape*
///             CollisionLayers::none()
///                 .with_group(GameLayer::Player) // <-- Mark it as the player
///                 .with_masks(&[GameLayer::World, GameLayer::Enemies]) // <-- Defines that the player collides with world and enemies (but not with other players)
///         );
/// }
/// ```
#[derive(Debug, Component, Copy, Clone, Eq, PartialEq, Reflect)]
pub struct CollisionLayers {
    groups: u32,
    masks: u32,
}

impl Default for CollisionLayers {
    fn default() -> Self {
        Self {
            groups: 0xffff_ffff,
            masks: 0xffff_ffff,
        }
    }
}

impl CollisionLayers {
    /// Create a new collision layers configuration with a single group and mask.
    ///
    /// You may add more groups and mask with `with_group` and `with_mask`.
    #[must_use]
    pub fn new<L: PhysicsLayer>(group: L, mask: L) -> Self {
        Self::from_bits(group.to_bits(), mask.to_bits())
    }

    /// Contains all groups and masks
    ///
    /// The entity, will interacts with everything (except the entities that interact with
    /// nothing).
    #[must_use]
    pub fn all<L: PhysicsLayer>() -> Self {
        Self::from_bits(L::all_bits(), L::all_bits())
    }

    /// Contains all groups and no masks
    ///
    /// The entity, will not interact with anything, unless you add masks via [`CollisionLayers::with_mask`]. You
    /// can also exclude specific groups using [`CollisionLayers::without_group`].
    #[must_use]
    pub fn all_groups<L: PhysicsLayer>() -> Self {
        Self::from_bits(L::all_bits(), 0)
    }

    /// Contains no groups and all masks
    ///
    /// The entity, will not interact with anything, unless you add group via [`CollisionLayers::with_group`]. You
    /// can also exclude specific masks using [`CollisionLayers::without_mask`].
    #[must_use]
    pub fn all_masks<L: PhysicsLayer>() -> Self {
        Self::from_bits(0, L::all_bits())
    }

    /// Contains no masks and groups
    ///
    /// The entity, will not interact with anything
    #[must_use]
    pub const fn none() -> Self {
        Self::from_bits(0, 0)
    }

    #[must_use]
    #[allow(missing_docs)]
    pub const fn from_bits(groups: u32, masks: u32) -> Self {
        Self { groups, masks }
    }

    /// Returns true if the entity would interact with an entity containing the `other` [`CollisionLayers]`
    #[must_use]
    pub fn interacts_with(self, other: Self) -> bool {
        (self.groups & other.masks) != 0 && (other.groups & self.masks) != 0
    }

    /// Returns true if the given layer is contained in the "groups"
    #[must_use]
    pub fn contains_group(self, layer: impl PhysicsLayer) -> bool {
        (self.groups & layer.to_bits()) != 0
    }

    /// Add the given layer in the "groups"
    #[must_use]
    pub fn with_group(mut self, layer: impl PhysicsLayer) -> Self {
        self.groups |= layer.to_bits();
        self
    }

    /// Add the given layers in the "groups"
    #[must_use]
    pub fn with_groups(mut self, layers: impl IntoIterator<Item = impl PhysicsLayer>) -> Self {
        for layer in layers.into_iter().map(|l| l.to_bits()) {
            self.groups |= layer;
        }

        self
    }

    /// Remove the given layer from the "groups"
    #[must_use]
    pub fn without_group(mut self, layer: impl PhysicsLayer) -> Self {
        self.groups &= !layer.to_bits();
        self
    }

    /// Returns true if the given layer is contained in the "masks"
    #[must_use]
    pub fn contains_mask(self, layer: impl PhysicsLayer) -> bool {
        (self.masks & layer.to_bits()) != 0
    }

    /// Add the given layer in the "masks"
    #[must_use]
    pub fn with_mask(mut self, layer: impl PhysicsLayer) -> Self {
        self.masks |= layer.to_bits();
        self
    }

    /// Add the given layers in the "masks"
    #[must_use]
    pub fn with_masks(mut self, layers: impl IntoIterator<Item = impl PhysicsLayer>) -> Self {
        for layer in layers.into_iter().map(|l| l.to_bits()) {
            self.masks |= layer;
        }

        self
    }

    /// Remove the given layer from the "masks"
    #[must_use]
    pub fn without_mask(mut self, layer: impl PhysicsLayer) -> Self {
        self.masks &= !layer.to_bits();
        self
    }

    #[must_use]
    #[allow(missing_docs)]
    pub fn groups_bits(self) -> u32 {
        self.groups
    }

    #[must_use]
    #[allow(missing_docs)]
    pub fn masks_bits(self) -> u32 {
        self.masks
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    enum TestLayer {
        One,
        Two,
    }

    impl PhysicsLayer for TestLayer {
        fn to_bits(&self) -> u32 {
            match self {
                TestLayer::One => 1,
                TestLayer::Two => 2,
            }
        }

        fn all_bits() -> u32 {
            3
        }
    }

    #[test]
    fn all_interacts_with_all() {
        assert!(
            CollisionLayers::all::<TestLayer>().interacts_with(CollisionLayers::all::<TestLayer>())
        );
    }

    #[rstest]
    #[case(CollisionLayers::all::<TestLayer>())]
    #[case(CollisionLayers::none())]
    #[case(CollisionLayers::all_groups::<TestLayer>())]
    #[case(CollisionLayers::all_masks::<TestLayer>())]
    fn none_does_not_interact_with_any_anything(#[case] other: CollisionLayers) {
        assert!(!CollisionLayers::none().interacts_with(other));
        assert!(!other.interacts_with(CollisionLayers::none()));
    }

    #[test]
    fn empty_groups_and_masks_not_interact() {
        let c1 = CollisionLayers::all_groups::<TestLayer>();
        let c2 = CollisionLayers::all_masks::<TestLayer>();

        assert!(!c1.interacts_with(c2));
        assert!(!c2.interacts_with(c1));
    }

    #[test]
    fn with_layer_adds_interaction() {
        let c1 = CollisionLayers::none()
            .with_group(TestLayer::One)
            .with_mask(TestLayer::Two);

        let c2 = CollisionLayers::none()
            .with_group(TestLayer::Two)
            .with_mask(TestLayer::One);

        assert!(c1.interacts_with(c2));
        assert!(c2.interacts_with(c1));
        assert!(!c1.interacts_with(c1));
        assert!(!c2.interacts_with(c2));
    }

    #[test]
    fn without_layer_removes_interaction() {
        let c1 = CollisionLayers::all::<TestLayer>()
            .without_group(TestLayer::One)
            .without_mask(TestLayer::Two);

        let c2 = CollisionLayers::all::<TestLayer>()
            .without_group(TestLayer::Two)
            .without_mask(TestLayer::One);

        println!("{:?}, {:?}", c1, c2);
        assert!(c1.interacts_with(c2));
        assert!(c2.interacts_with(c1));
        assert!(!c1.interacts_with(c1));
        assert!(!c2.interacts_with(c2));
    }
}
