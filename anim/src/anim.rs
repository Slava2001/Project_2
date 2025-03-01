use builder::{config::Config, BuildFromCfg};
use error_stack::Result;

pub struct Anim {

}

impl Anim {

}

impl BuildFromCfg for Anim {
    fn build(cfg: Config, resources: &mut dyn resources::Manager) -> Result<Anim, builder::Error> {

    }
}
