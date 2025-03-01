//! Single animation. Used to manage frame sequence.

use builder::config::Config;
use error_stack::{ensure, Result, ResultExt};
use scene::TimeTick;
use utils::rect::Rectf;

/// Single animation.
pub struct Anim {
    /// Frames array.
    frames: Vec<Rectf>,
    /// Current frame index.
    current_frame: usize,
    /// One frame duration in [`TimeTick`].
    frame_time: TimeTick,
    /// [`TimeTick`] elapsed since the previous frame.
    elapsed_time: TimeTick,
}

impl Anim {
    /// Create new Animation from config.
    ///
    /// # Errors
    /// Return error if config is not valid.
    pub fn new(mut cfg: Config) -> Result<Self, builder::Error> {
        let frames: Vec<Rectf> = cfg
            .take("frames")
            .change_context(builder::Error::msg("Failed to init anim frames array"))?;
        ensure!(
            !frames.is_empty(),
            builder::Error::msg("Failed to init anim frames: array is empty")
        );
        let frame_time = cfg
            .take("frame_time")
            .change_context(builder::Error::msg("Failed to init anim frame time"))?;
        Ok(Self { frames, current_frame: 0, frame_time, elapsed_time: 0 })
    }

    /// Update animation.
    /// Returns true if the animation has ended and started again.
    pub fn update(&mut self, delta_time: TimeTick) -> bool {
        self.elapsed_time += delta_time;
        let elapsed_frames = self.elapsed_time / self.frame_time;
        self.elapsed_time %= self.frame_time;
        self.current_frame += elapsed_frames;
        let rc = self.current_frame >= self.frames.len();
        if rc {
            self.current_frame %= self.frames.len();
        }
        rc
    }

    /// Reset animation.
    pub fn reset(&mut self) {
        self.elapsed_time = 0;
        self.current_frame = 0;
    }

    /// Get current texture frame.
    #[must_use]
    pub fn get_rect(&self) -> &Rectf {
        &self.frames[self.current_frame]
    }
}
