use bevy::reflect::Reflect;

pub trait Layer: Sized {
    fn from_bits(bits: u16) -> Self;
    fn to_bits(&self) -> u16;
}

/// Components that defines the collision layers of the collision shape.
///
/// This component must be on the same entity of a [`CollisionShape`](crate::CollisionShape)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Reflect)]
pub struct CollisionLayers {
    groups: u16,
    masks: u16,
}

impl Default for CollisionLayers {
    fn default() -> Self {
        Self::all()
    }
}

impl CollisionLayers {
    pub fn new<L: Layer>(groups: L, masks: L) -> Self {
        Self {
            groups: groups.to_bits(),
            masks: masks.to_bits(),
        }
    }

    pub fn all() -> Self {
        Self {
            groups: 0xffff,
            masks: 0xffff,
        }
    }

    pub fn none() -> Self {
        Self {
            groups: 0,
            masks: 0,
        }
    }

    pub fn interacts_with(self, other: Self) -> bool {
        (self.groups & other.masks) != 0 && (other.groups & self.masks) != 0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn all_interacts_with_all() {
        assert!(CollisionLayers::all().interacts_with(CollisionLayers::all()))
    }

    #[rstest]
    #[case(CollisionLayers::all())]
    #[case(CollisionLayers::none())]
    fn none_does_not_interact_with_any_anything(#[case] other: CollisionLayers) {
        assert!(!CollisionLayers::none().interacts_with(other))
    }
}
