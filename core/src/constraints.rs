use bevy::reflect::Reflect;

/// Component that restrict the rotations caused by forces
///
/// Note that angular velocity may still be applied programmatically.
#[derive(Debug, Copy, Clone, Reflect)]
pub struct RotationConstraints {
    /// Set to true to prevent rotations around the x axis
    pub allow_x: bool,

    /// Set to true to prevent rotations around the y axis
    pub allow_y: bool,

    /// Set to true to prevent rotations around the Z axis
    pub allow_z: bool,
}

impl Default for RotationConstraints {
    fn default() -> Self {
        Self::allow()
    }
}

impl RotationConstraints {
    /// Lock rotations around all axes
    #[must_use]
    pub fn lock() -> Self {
        Self {
            allow_x: false,
            allow_y: false,
            allow_z: false,
        }
    }

    /// Allow rotations around all axes
    #[must_use]
    pub fn allow() -> Self {
        Self {
            allow_x: true,
            allow_y: true,
            allow_z: true,
        }
    }

    /// Allow rotation around the x axis only (and prevent rotating around the other axes)
    #[must_use]
    pub fn restrict_to_x_only() -> Self {
        Self {
            allow_x: true,
            allow_y: false,
            allow_z: false,
        }
    }

    /// Allow rotation around the y axis only (and prevent rotating around the other axes)
    #[must_use]
    pub fn restrict_to_y_only() -> Self {
        Self {
            allow_x: false,
            allow_y: true,
            allow_z: false,
        }
    }

    /// Allow rotation around the z axis only (and prevent rotating around the other axes)
    #[must_use]
    pub fn restrict_to_z_only() -> Self {
        Self {
            allow_x: false,
            allow_y: false,
            allow_z: true,
        }
    }
}
