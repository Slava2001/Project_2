//! Simple resource manager implementation.

use error_stack::{bail, Result};
use opengl_graphics::{GlyphCache, Texture, TextureSettings};
use resources::{self, FontId, Manger, TextureId};
use std::collections::HashMap;
use std::path::Path;

/// Simple resource manager implementation.
pub struct ResMngr {
    /// All loaded textures.
    pub textures: Vec<Texture>,
    /// Map to associate texture string name with index in textures vector.
    textures_map: HashMap<String, TextureId>,
    /// All loaded fonts.
    pub fonts: Vec<GlyphCache<'static>>,
    /// Map to associate font string name with index in fonts vector.
    fonts_map: HashMap<String, FontId>,
}
impl ResMngr {
    /// Creates new resource manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            textures_map: HashMap::new(),
            fonts: Vec::new(),
            fonts_map: HashMap::new(),
        }
    }
}
impl Manger for ResMngr {
    fn load(&mut self, kind: &str, name: &str, path: &str) -> Result<(), resources::Error> {
        match kind {
            "texture" => {
                let id = TextureId(self.textures.len());
                let mut settings = TextureSettings::new();
                settings.set_filter(opengl_graphics::Filter::Nearest);
                self.textures.push(
                    Texture::from_path(Path::new(path), &settings).map_err(|e| {
                        resources::Error::msg(format!("Failed to load texture: {e}"))
                    })?,
                );
                self.textures_map.insert(name.into(), id);
                Ok(())
            }
            "font" => {
                let id = FontId(self.fonts.len());
                let mut settings = TextureSettings::new();
                settings.set_filter(opengl_graphics::Filter::Nearest);
                let cache = GlyphCache::new(path, (), TextureSettings::new())
                    .map_err(|e| resources::Error::msg(format!("Failed to load font: \"{e}\"")))?;
                self.fonts.push(cache);
                self.fonts_map.insert(name.into(), id);
                Ok(())
            }
            _ => bail!(resources::Error::msg(format!(
                "Failed to load recourse: unexpected resource \
                type: \"{kind}\", name: \"{name}\", path: \"{path}\""
            ))),
        }
    }

    fn get_texture(&self, name: &str) -> Result<TextureId, resources::Error> {
        Ok(*self
            .textures_map
            .get(name)
            .ok_or_else(|| resources::Error::msg(format!("Failed to find texture: \"{name}\"")))?)
    }

    fn get_font(&self, name: &str) -> Result<resources::FontId, resources::Error> {
        Ok(*self
            .fonts_map
            .get(name)
            .ok_or_else(|| resources::Error::msg(format!("Failed to find font: \"{name}\"")))?)
    }
}
