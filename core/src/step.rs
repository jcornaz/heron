use std::time::Duration;

use bevy::prelude::*;

use crate::utils::NearZero;

/// Resource to control how many physics steps are performed per second.
///
/// Note that a maximum of 1 step will be perform per physics update. It means that if the rate of
/// frames per second is lower than the physics step per second, the physics simulation will slows down.
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
    /// For good results, it is better to choose a value which is *lower* than the typical frame rate of the game.
    pub fn from_steps_per_seconds(steps_per_second: f32) -> Self {
        assert!(
            !steps_per_second.is_near_zero(),
            "Invalid steps per second: {}",
            steps_per_second
        );
        Self(Mode::Timer(Timer::from_seconds(
            1.0 / steps_per_second,
            true,
        )))
    }

    pub fn from_delta_time(duration: Duration) -> Self {
        assert_ne!(!duration.as_micros(), 0, "Invalid duration: {:?}", duration);
        Self(Mode::Timer(Timer::new(duration, true)))
    }

    pub fn every_frame(duration: Duration) -> Self {
        Self(Mode::EveryFrame(duration))
    }

    pub fn step_frame(&self) -> bool {
        match &self.0 {
            Mode::EveryFrame(_) => true,
            Mode::Timer(timer) => timer.just_finished(),
        }
    }

    pub fn duration(&self) -> Duration {
        match &self.0 {
            Mode::EveryFrame(duration) => *duration,
            Mode::Timer(timer) => timer.duration(),
        }
    }

    pub(crate) fn update(mut physics_steps: ResMut<'_, PhysicsSteps>, time: Res<'_, Time>) {
        if let Mode::Timer(timer) = &mut physics_steps.0 {
            timer.tick(time.delta());
        }
    }
}
