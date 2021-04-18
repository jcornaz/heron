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
///         .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
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
    fn pause(&mut self) {
        self.prev_time_scale = Some(self.time_scale);
        self.time_scale = 0.0;
    }

    fn resume(&mut self) {
        if self.time_scale == 0.0 {
            if let Some(prev) = self.prev_time_scale {
                self.time_scale = prev;
                self.prev_time_scale = None;
            }
        }
    }

    fn time_scale(&mut self, time_scale: f32) {
        if time_scale.is_sign_positive() {
            self.time_scale = time_scale;
        }
    }

    fn from(time_scale: f32) -> Self {
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
