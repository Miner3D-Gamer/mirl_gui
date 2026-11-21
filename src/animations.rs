//! Types that help with handling animations
//!
//! The main feature are the `get_frame_idx_from_{}` from [`FramesInfo`]

use mirl_extensions::*;

/// What should happen when the animation comes to an end
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnimationBehaviorOnEnd {
    /// Repeat from the start
    Repeat,
    /// Hold the last frame indefinitely
    HoldOnLastFrame,
    /// Ping pong style reverse
    Reverse,
}
/// Settings for animated frames
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnimatedSettings {
    /// The FPS
    pub _frames_per_second: mirl_narrowed_floats::NonZeroPositiveFiniteF32,
    /// The SPF
    pub _seconds_per_frame: std::time::Duration,
    /// What should happen when the animation idx overflows
    pub _behavior: AnimationBehaviorOnEnd,
    /// If the animation is currently reversed
    pub _reverse: bool,
    /// If the animation was originally reversed
    ///
    /// This value can change after the creation of the struct
    pub _originally_reverse: bool,
}
impl AnimatedSettings {
    // /// Create a new instance of
    // ///
    // /// # Errors
    // /// When the given `1/frames per second` are not convertible to a [`std::time::Duration`]
    // pub fn new_from_f32<T: IntoPatch<f32>>(
    //     frames_per_second: T,
    //     reverse: bool,
    //     behavior: AnimationBehaviorOnEnd,
    // ) -> Result<Self, std::time::TryFromFloatSecsError> {
    //     let f = frames_per_second.into_value();
    //     Ok(Self {
    //         _frames_per_second: f,
    //         _seconds_per_frame: std::time::Duration::try_from_secs_f32(1.0 / f)?,
    //         _behavior: behavior,
    //         _reverse: reverse,
    //         _originally_reverse: reverse,
    //     })
    // }
    #[must_use]
    /// Create a new instance of
    pub fn new_from_duration(
        seconds_per_frame: std::time::Duration,
        reverse: bool,
        behavior: AnimationBehaviorOnEnd,
    ) -> Option<Self> {
        Some(Self {
            _frames_per_second: mirl_narrowed_floats::NonZeroPositiveFinite::<_>::new(
                1.0 / seconds_per_frame.as_secs_f32(),
            )?,
            _seconds_per_frame: seconds_per_frame,
            _behavior: behavior,
            _reverse: reverse,
            _originally_reverse: reverse,
        })
    }
}
impl AnimatedSettings {
    /// Apply the internal settings to the given frame
    pub fn apply_settings_to_frame_idx(&mut self, frames: &FramesInfo, idx: usize) -> usize {
        let idx = match self._behavior {
            AnimationBehaviorOnEnd::Repeat => idx % frames._total,
            AnimationBehaviorOnEnd::HoldOnLastFrame => idx.min(frames._total),
            AnimationBehaviorOnEnd::Reverse => {
                let total_iterations = frames._total / idx;
                self._reverse = total_iterations
                    .is_multiple_of(2)
                    .invert_if(|_| self._originally_reverse);

                idx % frames._total
            }
        };
        if self._reverse {
            frames._total - idx
        } else {
            idx
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Info about given frames
pub struct FramesInfo {
    /// The total amount of fra,es
    pub _total: usize,
}
impl FramesInfo {
    /// Get the frame idx from a f32 duration
    pub fn get_frame_idx_from_f32(&self, settings: &mut AnimatedSettings, duration: f32) -> usize {
        let raw_frame = (duration % settings._seconds_per_frame.as_secs_f32()) as usize;
        settings.apply_settings_to_frame_idx(self, raw_frame)
    }
    /// Get the frame idx from
    pub fn get_frame_idx_from_instant(
        &self,
        settings: &mut AnimatedSettings,
        creation: std::time::Instant,
    ) -> usize {
        let now = std::time::Instant::now();
        let duration = now - creation;
        let raw_frame = duration.as_secs_f32() % settings._seconds_per_frame.as_secs_f32();
        self.get_frame_idx_from_f32(settings, raw_frame)
    }
}
