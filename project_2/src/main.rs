//! `Project_2` sandbox game.

mod scenes;

use builder::{BuildFromCfg, Config};
use error_stack::{Result, ResultExt};
use runtime::Runtime;
use scenes::{Level, MainMenu};

/// Window scale.
const WINDOW_SCALE: u32 = 50;
/// Window hight.
const WINDOW_H: u32 = 16 * WINDOW_SCALE;
/// Window width.
const WINDOW_W: u32 = 9 * WINDOW_SCALE;

/// `Project_2` error.
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct Error(String);
impl Error {
    /// Make error from message.
    fn msg<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

/// `Project_2` entry point.
fn main() {
    if let Err(e) = run() {
        println!("{e:?}");
    }
}

/// `Project_2` main loop.
fn run() -> Result<(), Error> {
    let runtime = Runtime::new("Project 2", (WINDOW_H, WINDOW_W))
        .change_context(Error::msg("Failed to init runtime"))?;
    let mut builder = scene::Builder::new();
    builder.reg_builder("main_menu", MainMenu::build);
    builder.reg_builder("level", Level::build);

    let scene_cfg = Config::from_file("assets/main_menu.json")
        .change_context(Error::msg("Failed to load scene config"))?;
    runtime.run(&builder, scene_cfg).change_context(Error::msg("Runtime error"))?;
    Ok(())
}
