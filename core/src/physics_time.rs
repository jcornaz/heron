/// Resource that controls the physics time scale
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
///
/// fn main() {
///     App::build()
///         // ... Add plugins
///         .insert_resource(PhysicsTime::from(0.5))
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

    /// Set the physics emulation time scale
    pub fn set_scale(&mut self, time_scale: f32) {
        if time_scale > 0.0 {
            self.scale = time_scale;
        }
    }

    /// Get the physics emulation time scale
    #[must_use]
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

impl From<f32> for PhysicsTime {
    fn from(time_scale: f32) -> Self {
        Self {
            scale: time_scale,
            previous_scale: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0.0)]
    #[case(1.0)]
    #[case(0.5)]
    #[case(-1.0)]
    fn pause_sets_scale_to_zero(#[case] initial_scale: f32) {
        let mut time = PhysicsTime::from(initial_scale);
        time.pause();
        assert_eq!(time.get_scale(), 0.0);
    }

    #[rstest]
    #[case(0.0)]
    #[case(1.0)]
    #[case(0.5)]
    #[case(-1.0)]
    fn pause_restore_time_scale_before_pause(#[case] initial_scale: f32) {
        let mut time = PhysicsTime::from(initial_scale);
        time.pause();
        time.resume();
        assert_eq!(time.get_scale(), initial_scale);
    }
}
