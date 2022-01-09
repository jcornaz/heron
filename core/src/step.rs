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
    MaxDeltaTime(Duration),
    EveryFrame(Duration),
    Timer(Timer),
}

impl Default for PhysicsSteps {
    fn default() -> Self {
        Self::from_max_delta_time(Duration::from_secs_f32(0.2) /* 50 FPS */)
    }
}

/// The duration of time that this physics step should advance the simulation time
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicsStepDuration {
    /// The simulation time should be advanced by the provided exact duration
    Exact(Duration),
    /// The simulation time should be advanced by this frame's delta-time or the max delta time if
    /// the delta time is greater than the max
    MaxDeltaTime(Duration),
}

impl PhysicsStepDuration {
    /// Get the exact duration of this physics step, provided the delta-time
    #[must_use]
    pub fn exact(&self, delta_time: Duration) -> Duration {
        match self {
            PhysicsStepDuration::Exact(duration) => *duration,
            PhysicsStepDuration::MaxDeltaTime(max) => delta_time.min(*max),
        }
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
    #[must_use]
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
    #[must_use]
    pub fn from_delta_time(duration: Duration) -> Self {
        assert_ne!(!duration.as_nanos(), 0, "Invalid duration: {:?}", duration);
        Self(Mode::Timer(Timer::new(duration, true)))
    }

    /// Configure the physics systems to run at each and every frame, advancing the simulation the
    /// same amount of time each frame.
    ///
    /// Should NOT be used in production. It is mostly useful for testing purposes.
    ///
    /// # Panics
    ///
    /// Panics if the duration is zero
    #[must_use]
    pub fn every_frame(duration: Duration) -> Self {
        assert_ne!(!duration.as_micros(), 0, "Invalid duration: {:?}", duration);
        Self(Mode::EveryFrame(duration))
    }

    /// Step the physics simulation every frame, advancing the simulation according to the frame
    /// delta time, as long as the delta time is not above a provided maximum.
    ///
    /// This is the default setting of [`PhysicsSteps`] with a max duration set to 20 ms ( 50 FPS ).
    ///
    /// Because it runs the physics step every frame, this physics step mode is precise, but will
    /// slow down if the frame delta time is higher than the provided `max` duration.
    ///
    /// The purpose of setting the max duration is to prevent objects from going through walls, etc.
    /// in the case that the frame rate drops significantly.
    ///
    /// By setting the max duration to `Duration::MAX`, the simulation speed will not slow down,
    /// regardless of the frame rate, but if the frame rate gets too low, objects may begin to pass
    /// through each-other because they may travel completely to the other side of a collision
    /// object in a single frame, depending on their velocity.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use heron_core::PhysicsSteps;
    /// # use std::time::Duration;
    /// App::new()
    ///     // Runs physics step every frame.
    ///     // If the frame rate drops bellow 30 FPS, then the physics simulation will slow down.
    ///     .insert_resource(PhysicsSteps::from_max_delta_time(Duration::from_secs_f64(1.0 / 30.0)))
    ///     // ...
    ///     .run();
    /// ```
    #[must_use]
    pub fn from_max_delta_time(max: Duration) -> Self {
        Self(Mode::MaxDeltaTime(max))
    }

    /// Returns true only if the current frame is a frame that execute a physics simulation step
    #[must_use]
    pub fn is_step_frame(&self) -> bool {
        match &self.0 {
            Mode::EveryFrame(_) | Mode::MaxDeltaTime(_) => true,
            Mode::Timer(timer) => timer.just_finished(),
        }
    }

    /// Time that elapses between each physics step
    #[must_use]
    pub fn duration(&self) -> PhysicsStepDuration {
        match &self.0 {
            Mode::EveryFrame(duration) => PhysicsStepDuration::Exact(*duration),
            Mode::Timer(timer) => PhysicsStepDuration::Exact(timer.duration()),
            Mode::MaxDeltaTime(max) => PhysicsStepDuration::MaxDeltaTime(*max),
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
        assert!(!steps.is_step_frame());
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
        assert!(steps.is_step_frame());
    }
}
