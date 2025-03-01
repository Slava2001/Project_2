use builder::config::Config;
use error_stack::{ensure, Result, ResultExt};
use scene::TimeTick;
use utils::rect::Rectf;

pub struct Anim {
    frames: Vec<Rectf>,
    current_frame: usize,
    frame_time: TimeTick,
    elapsed_time: TimeTick,
}

impl Anim {
    pub fn new(mut cfg: Config) -> Result<Anim, builder::Error> {
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

    pub fn update(&mut self, delta_time: TimeTick) -> bool {
        self.elapsed_time += delta_time;
        let elapsed_frames = self.elapsed_time / self.frame_time;
        self.elapsed_time = self.elapsed_time % self.frame_time;
        self.current_frame += elapsed_frames;
        let rc = self.current_frame >= self.frames.len();
        if rc {
            self.current_frame = self.current_frame % self.frames.len();
        }
        rc
    }

    pub fn reset(&mut self) {
        self.elapsed_time = 0;
        self.current_frame = 0;
    }

    pub fn get_rect(&self) -> &Rectf {
        &self.frames[self.current_frame]
    }
}
