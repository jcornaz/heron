use std::time::Duration;

use bevy::prelude::*;

/// Resource to control how many physics steps are performed per second.
///
/// Note that the physics update will be performed at most once per frame. It means that if the rate of
/// frames per second is lower than the physics step per second, the physics simulation will slows down.
///
/// This resource is used to tune the precision and performance of the physics system.
/// It doesn't change the speed of the simulation.
/// To change the time scale, look at the [`PhysicsTime`](crate::PhysicsTime) resource instead.
pub struct PhysicsSteps(Mode);

enum Mode {
    EveryFrame(Duration),
    Timer(Timer),
}

impl Default for PhysicsSteps {
    fn default() -> Self {
        Self::from_steps_per_seconds(58.0)
    }
}

impl PhysicsSteps {
    /// Configure to run at the given number of steps per second
    ///
    /// The higher the value, the more precise and the more expensive the physics simulation will be.
    /// If the value gets higher than the frame rate of the game, the physics simulation will slows down.
    ///
    /// For good results, it is better to choose a value as high as possible but lower than the typical frame rate of the game.
    ///
    /// # Panics
    ///
    /// Panics if the argument is nan, infinite or negative
    pub fn from_steps_per_seconds(steps_per_second: f32) -> Self {
        assert!(
            steps_per_second.is_finite() && steps_per_second > 0.0,
            "Invalid steps per second: {}",
            steps_per_second
        );
        Self(Mode::Timer(Timer::from_seconds(
            1.0 / steps_per_second,
            true,
        )))
    }

    /// Configure the physics systems to wait for the given duration before running again
    ///
    /// The lower the value, the more precise and the more expensive the physics simulation will be.
    /// If the value gets lower than the delta time between each frame of the game, the physics simulation will slows down.
    ///
    /// For good results, it is better to choose a value as low as possible, but higher than the typical delay between each frame of the game.
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    pub fn from_delta_time(duration: Duration) -> Self {
        assert_ne!(!duration.as_nanos(), 0, "Invalid duration: {:?}", duration);
        Self(Mode::Timer(Timer::new(duration, true)))
    }

    /// Configure the physics systems to run at each and every frame. Regardless of the current FPS.
    ///
    /// It takes a duration which is "haw much" the physics simulation should advance at each frame.
    ///
    /// Should NOT be used in production. It is mostly useful for testing purposes.
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    pub fn every_frame(duration: Duration) -> Self {
        assert_ne!(!duration.as_micros(), 0, "Invalid duration: {:?}", duration);
        Self(Mode::EveryFrame(duration))
    }

    /// Returns true only if the current frame is a frame that execute a physics simulation step
    pub fn is_step_frame(&self) -> bool {
        match &self.0 {
            Mode::EveryFrame(_) => true,
            Mode::Timer(timer) => timer.just_finished(),
        }
    }

    /// Time that elapses between each physics step
    pub fn duration(&self) -> Duration {
        match &self.0 {
            Mode::EveryFrame(duration) => *duration,
            Mode::Timer(timer) => timer.duration(),
        }
    }

    pub(crate) fn update(mut physics_steps: ResMut<'_, PhysicsSteps>, time: Res<'_, Time>) {
        physics_steps.do_update(time.delta());
    }

    #[inline]
    fn do_update(&mut self, delta: Duration) {
        if let Mode::Timer(timer) = &mut self.0 {
            timer.tick(delta);
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(PhysicsSteps::from_delta_time(Duration::from_secs(1)), 0.9)]
    #[case(PhysicsSteps::from_delta_time(Duration::from_secs_f32(0.016)), 0.01)]
    #[case(PhysicsSteps::from_steps_per_seconds(10.0), 0.09)]
    fn is_not_step_frame_if_not_enough_time_has_elapsed(
        #[case] mut steps: PhysicsSteps,
        #[case] delta_time: f32,
    ) {
        steps.do_update(Duration::from_secs_f32(delta_time));
        assert!(!steps.is_step_frame())
    }

    #[rstest]
    #[case(PhysicsSteps::from_delta_time(Duration::from_secs(1)), 1.01)]
    #[case(PhysicsSteps::from_delta_time(Duration::from_secs_f32(0.016)), 0.017)]
    #[case(PhysicsSteps::from_steps_per_seconds(10.0), 0.11)]
    #[case(PhysicsSteps::every_frame(Duration::from_secs(1)), 1.1)]
    #[case(PhysicsSteps::every_frame(Duration::from_secs(1)), 0.9)]
    fn is_step_frame_when_enough_time_has_elapsed(
        #[case] mut steps: PhysicsSteps,
        #[case] delta_time: f32,
    ) {
        steps.do_update(Duration::from_secs_f32(delta_time));
        assert!(steps.is_step_frame())
    }
}
