/// Resource that controls the physics time scale
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
///
/// fn main() {
///     App::new()
///         // ... Add plugins
///         .insert_resource(PhysicsTime::new(0.5))
///         // ... Add systems
///         .run();
/// }
/// ```
#[derive(Debug, Copy, Clone)]
pub struct PhysicsTime {
    /// Specify the physics emulation time scale used
    scale: f32,
    previous_scale: Option<f32>,
}

impl PhysicsTime {
    /// Create a new physics time for the given scale (which must be >= 0).
    ///
    /// # Panics
    ///
    /// Panic if the scale is negative.
    ///
    #[must_use]
    pub fn new(scale: f32) -> Self {
        assert!(scale >= 0.0, "Negative scale: {}", scale);
        Self {
            scale,
            previous_scale: None,
        }
    }

    /// Pause the physics emulation, avoiding heron systems to run.
    pub fn pause(&mut self) {
        self.previous_scale = Some(self.scale);
        self.scale = 0.0;
    }

    /// Resume the physics emulation
    pub fn resume(&mut self) {
        if let Some(prev) = self.previous_scale {
            self.scale = prev;
            self.previous_scale = None;
        }
    }

    /// Set the physics emulation time scale (must be positive)
    ///
    /// # Panics
    ///
    /// Panic if the scale is negative
    ///
    pub fn set_scale(&mut self, scale: f32) {
        assert!(scale >= 0.0);
        self.scale = scale;
    }

    /// Get the physics emulation time scale
    #[must_use]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Get the physics emulation time scale
    #[must_use]
    #[deprecated(note = "Please use 'scale()' instead")]
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
}

impl Default for PhysicsTime {
    fn default() -> Self {
        Self {
            scale: 1.0,
            previous_scale: None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0.0)]
    #[case(1.0)]
    #[case(0.5)]
    fn pause_sets_scale_to_zero(#[case] initial_scale: f32) {
        let mut time = PhysicsTime::new(initial_scale);
        time.pause();
        assert_eq!(time.scale(), 0.0);
    }

    #[rstest]
    #[case(0.0)]
    #[case(1.0)]
    #[case(0.5)]
    fn pause_restore_time_scale_before_pause(#[case] initial_scale: f32) {
        let mut time = PhysicsTime::new(initial_scale);
        time.pause();
        time.resume();
        assert_eq!(time.scale(), initial_scale);
    }

    #[rstest]
    #[case(1.0, 2.0)]
    #[case(1.0, 0.0)]
    #[case(1.0, 1.0)]
    #[case(0.0, 0.0)]
    fn scale_can_be_set(#[case] initial_scale: f32, #[case] new_scale: f32) {
        let mut time = PhysicsTime::new(initial_scale);
        time.set_scale(new_scale);
        assert_eq!(new_scale, time.scale());
    }

    #[rstest]
    #[case(PhysicsTime::new(1.0), -1.0)]
    #[case(PhysicsTime::new(0.0), -0.1)]
    #[should_panic]
    fn set_negative_scale_panics(#[case] mut time: PhysicsTime, #[case] new_scale: f32) {
        time.set_scale(new_scale);
    }

    #[rstest]
    #[case(-1.0)]
    #[case(-0.1)]
    #[should_panic]
    fn new_with_negative_scale_panics(#[case] scale: f32) {
        let _ = PhysicsTime::new(scale);
    }
}
