//! Animation.
//! State machine in which each state corresponds to an animation.

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

/// Animator static config declare helper. It used to configure animator FSM.
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
                        make_animator_cfg!(
                            @inner, line, $event_enum, $event, $state_enum, $new_state
                        );
                    )*
                    transient_map.insert(<$state_enum>::$state, line);

                    struct UniqueArgsChecker<$($event),*> {
                        _phantom: ::std::marker::PhantomData<($($event,)*)>
                    }
                }
            )*
            struct UniqueArgsChecker<$($state),*> {
                _phantom: ::std::marker::PhantomData<($($state,)*)>
            }
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
    (@inner, $map:ident, $key_enum:ty, $key:ident, $enum_ty:ty, None) => {
        $map.insert(<$key_enum>::$key, None);
    };
    (@inner, $map:ident, $key_enum:ty, $key:ident, $enum_ty:ty, $val:ident) => {
        $map.insert(<$key_enum>::$key, Some(<$enum_ty>::$val));
    };
}

/// Animator static config, use [`make_animator_cfg`] for create it.
pub struct AnimatorCfg<S, E> {
    /// Init state.
    pub state: S,
    /// Transients map.
    pub transient_map: HashMap<S, HashMap<E, Option<S>>>,
    /// Animations names for each state.
    pub anim_names: Vec<(&'static str, S)>,
    /// Anim end event.
    pub timeout_event: E,
}

/// Animator.
pub struct Animator<S: Eq + Hash + Copy + Debug, E: Eq + Hash + Copy + Debug> {
    /// Init state.
    state: S,
    /// Transients map.
    transient_map: HashMap<S, HashMap<E, Option<S>>>,
    /// List of animations.
    anims: HashMap<S, Anim>,
    /// Ent of animation event.
    timeout_event: E,
    /// Sprite sheet.
    texture: TextureId,
    /// Animator draw rect.
    rect: Rectf,
    /// Current texture rect.
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
                .ok_or_else(|| builder::Error::msg(format!("Required anim {k:?} not found")))?;
            if anims.insert(v, a).is_some() {
                bail!(builder::Error::msg(format!("State {v:?} duplicated in transients map")));
            }
        }

        let texture_rect = *anims
            .get(&animator_cfg.state)
            .ok_or_else(|| {
                report!(builder::Error::msg(format!(
                    "Failed to get animation for state: {:?}",
                    animator_cfg.state
                )))
            })?
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

    /// Handle event.
    ///
    /// # Errors
    /// Return error if transient from current state by specified event not found.
    pub fn handle_event(&mut self, e: E) -> Result<(), Error> {
        let Some(next_state) = self.transient_map.get(&self.state).and_then(|l| l.get(&e)) else {
            bail!(Error::msg(format!(
                "Failed to get transient from state {:?} by event {:?}",
                self.state, e
            )));
        };
        if let Some(next_state) = next_state {
            self.anims
                .get_mut(&self.state)
                .ok_or_else(|| {
                    Error::msg(format!("Failed to get animation for state: {next_state:?}"))
                })?
                .reset();
            self.state = *next_state;
        }
        Ok(())
    }

    /// Update animator.
    ///
    /// # Errors
    /// Return error if failed to update current animation.
    pub fn update(&mut self, dt: TimeTick) -> Result<(), Error> {
        let anim = self.anims.get_mut(&self.state).ok_or_else(|| {
            Error::msg(format!("Failed to get animation for state: {:?}", self.state))
        })?;
        let new_cycle = anim.update(dt);
        self.texture_rect = *anim.get_rect();
        if new_cycle {
            self.handle_event(self.timeout_event)?;
        }
        Ok(())
    }

    /// Set animator position.
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

impl<S: Eq + Hash + Copy + Debug, E: Eq + Hash + Copy + Debug> Debug for Animator<S, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Animator: ")?;
        write!(f, "digraph Anim {{")?;
        write!(f, "{:?} [color = green];", self.state)?;
        for (from_state, transients) in &self.transient_map {
            for (by_event, to_state) in transients {
                if let Some(to_state) = to_state {
                    write!(f, "{from_state:?} -> {to_state:?} [label=\"{by_event:?}  \"];")?;
                } else {
                    write!(
                        f,
                        "{from_state:?} -> {from_state:?} [label=\"{by_event:?}  \" style=dotted];"
                    )?;
                }
            }
        }
        writeln!(f, "}}")
    }
}
