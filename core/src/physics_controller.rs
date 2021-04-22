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
    time_scale: f32,
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

    /// Set the physics emulation time scale
    pub fn time_scale(&mut self, time_scale: f32) {
        if time_scale.is_sign_positive() {
            self.time_scale = time_scale;
        }
    }

    /// Get the physics emulation time scale
    pub fn current_time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Initialize a `PhysicsController` struct with an initial time scale
    pub fn from(time_scale: f32) -> Self {
        let mut physics_controller = Self::default();
        physics_controller.time_scale(time_scale);
        physics_controller
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
