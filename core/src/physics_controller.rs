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
    pub scale: f32,
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
        if self.scale == 0.0 {
            if let Some(prev) = self.previous_scale {
                self.scale = prev;
                self.previous_scale = None;
            }
        }
    }

    /// Initialize a `PhysicsController` struct with an initial time scale
    #[must_use]
    pub fn from(time_scale: f32) -> Self {
        Self {
            scale: time_scale,
            previous_scale: None,
        }
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
