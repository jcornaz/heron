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
///         .insert_resource(PhysicsController::from(0.5))
///         // ... Add systems
///         .run();
/// }
/// ```
#[derive(Debug, Copy, Clone)]
pub struct PhysicsController {
    /// Specify the physics emulation time scale used
    pub time_scale: f32,
    prev_time_scale: Option<f32>,
}

impl PhysicsController {
    /// Pause the physics emulation, avoiding heron systems to run.
    pub fn pause(&mut self) {
        self.prev_time_scale = Some(self.time_scale);
        self.time_scale = 0.0;
    }

    /// Resume the physics emulation
    pub fn resume(&mut self) {
        if self.time_scale == 0.0 {
            if let Some(prev) = self.prev_time_scale {
                self.time_scale = prev;
                self.prev_time_scale = None;
            }
        }
    }

    /// Initialize a `PhysicsController` struct with an initial time scale
    #[must_use]
    pub fn from(time_scale: f32) -> Self {
        Self {
            time_scale,
            prev_time_scale: None,
        }
    }
}

impl Default for PhysicsController {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            prev_time_scale: None,
        }
    }
}
