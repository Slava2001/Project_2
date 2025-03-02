//! Animation.

use anim::Anim;
use builder::config::Config;
use error_stack::{bail, report, Result, ResultExt};
use renderer::Drawable;
use resources::TextureId;
use scene::TimeTick;
use std::{collections::HashMap, fmt::Debug, hash::Hash, path::PathBuf};
use utils::{rect::Rectf, vec2::Vec2f};

pub mod anim;

/// Parse error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
impl Error {
    /// Make error from message.
    pub fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

#[macro_export]
macro_rules! make_animator_cfg {
    (
        State_enum: $state_enum:ty,
        Event_enum: $event_enum:ty,
        Init_state: $init_state:ident,
        Anim_fin_event: $anim_fin_event:ident,
        Anim_map:
            $($anim_state:ident: $anim_name:literal),*
        Transient_map:
            $($state:ident:
                $($event:ident => $new_state:ident),*
            );*
    ) => {
        {
            let mut anim_names = Vec::new();
            {
                $(anim_names.push(($anim_name, <$state_enum>::$anim_state));)*
            }
            match <$state_enum>::$init_state {
                $(<$state_enum>::$anim_state => {})*
            }
            let mut transient_map = ::std::collections::HashMap::new();
            $(
                {
                    let mut line = ::std::collections::HashMap::new();
                    $(
                        line.insert(<$event_enum>::$event, <$state_enum>::$new_state);
                    )*
                    transient_map.insert(<$state_enum>::$state, line);
                }
            )*
            match <$state_enum>::$init_state {
                $(<$state_enum>::$state => {})*
            }
            $crate::AnimatorCfg {
                state: <$state_enum>::$init_state,
                transient_map,
                anim_names,
                timeout_event: <$event_enum>::$anim_fin_event
            }
        }
    };
}

pub struct AnimatorCfg<S, E> {
    pub state: S,
    pub transient_map: HashMap<S, HashMap<E, S>>,
    pub anim_names: Vec<(&'static str, S)>,
    pub timeout_event: E,
}

pub struct Animator<S: Eq + Hash + Copy + Debug, E: Eq + Hash + Copy + Debug> {
    state: S,
    transient_map: HashMap<S, HashMap<E, S>>,
    anims: HashMap<S, Anim>,
    timeout_event: E,
    texture: TextureId,
    rect: Rectf,
    texture_rect: Rectf,
}

impl<S: Eq + Hash + Copy + Debug, E: Eq + Hash + Copy + Debug> Animator<S, E> {
    /// Create new animator from config.
    ///
    /// # Errors
    /// Return error if config is not valid.
    pub fn new(
        animator_cfg: AnimatorCfg<S, E>,
        mut cfg: Config,
        res: &mut dyn resources::Manager,
    ) -> Result<Self, builder::Error> {
        let texture_path = cfg
            .take::<PathBuf>("texture")
            .change_context(builder::Error::msg("Failed to init texture name"))?;
        let texture_name = texture_path.display().to_string();
        res.load("texture", &texture_name, &texture_path)
            .change_context(builder::Error::msg("Failed to load texture"))?;
        let texture = res
            .get_texture(&texture_name)
            .change_context(builder::Error::msg("Failed to get texture from resources manager"))?;
        let mut anims_cfg = cfg
            .take::<HashMap<String, Anim>>("anims")
            .change_context(builder::Error::msg("Failed to load animations"))?;
        let rect =
            cfg.take("rect").change_context(builder::Error::msg("Failed to init animator rect"))?;

        let mut anims = HashMap::new();
        for (k, v) in animator_cfg.anim_names {
            let a = anims_cfg
                .remove(k)
                .ok_or(builder::Error::msg(format!("Required anim {k:?} not found")))?;
            if anims.insert(v, a).is_some() {
                bail!(builder::Error::msg(format!("State {v:?} duplicated in transients map")));
            }
        }

        let texture_rect = *anims
            .get(&animator_cfg.state)
            .ok_or(report!(builder::Error::msg(format!(
                "Failed to get animation for state: {:?}",
                animator_cfg.state
            ))))?
            .get_rect();

        Ok(Self {
            rect,
            anims,
            texture,
            texture_rect,
            state: animator_cfg.state,
            timeout_event: animator_cfg.timeout_event,
            transient_map: animator_cfg.transient_map,
        })
    }

    pub fn handle_event(&mut self, e: E) -> Result<(), Error> {
        if let Some(s) = self.transient_map.get(&self.state).and_then(|l| l.get(&e)) {
            self.state = *s;
            self.anims
                .get_mut(&self.state)
                .ok_or(Error::msg(format!("Failed to get animation for state: {:?}", self.state)))?
                .reset();
        }
        Ok(())
    }

    pub fn update(&mut self, dt: TimeTick) -> Result<(), Error> {
        let anim = self
            .anims
            .get_mut(&self.state)
            .ok_or(Error::msg(format!("Failed to get animation for state: {:?}", self.state)))?;
        let new_cycle = anim.update(dt);
        self.texture_rect = *anim.get_rect();
        if new_cycle {
            self.handle_event(self.timeout_event)?;
        }
        Ok(())
    }

    pub fn set_pos(&mut self, pos: Vec2f) {
        self.rect.x = pos.x;
        self.rect.y = pos.y;
    }
}

impl<S: Eq + Hash + Copy + Debug, E: Eq + Hash + Copy + Debug> Drawable for Animator<S, E> {
    fn draw(&self, renderer: &mut dyn renderer::Renderer) {
        renderer.draw_img(&self.rect, self.texture, &self.texture_rect);
    }
}
