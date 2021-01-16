use bevy_math::{Vec2, Vec3};

/// Resource that defines world's gravity.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gravity(Vec3);

impl Gravity {
    /// Returns the underlying vector
    pub fn vector(self) -> Vec3 {
        self.0
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self::from(Vec3::zero())
    }
}

impl From<Vec3> for Gravity {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl From<Vec2> for Gravity {
    fn from(v: Vec2) -> Self {
        Self::from(v.extend(0.0))
    }
}

impl From<Gravity> for Vec3 {
    fn from(g: Gravity) -> Self {
        g.vector()
    }
}
