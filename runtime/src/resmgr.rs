use std::collections::HashMap;
use std::path::Path;
use error_stack::{bail, Result};
use opengl_graphics::{GlyphCache, Texture, TextureSettings};
use resources::{self, FontId, Manger, TextureId};

pub struct ResMngr {
    pub textures: Vec<Texture>,
    textures_map: HashMap<String, TextureId>,
    pub fonts: Vec<GlyphCache<'static>>,
    fonts_map: HashMap<String, FontId>,
}
impl ResMngr {
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
            .ok_or(resources::Error::msg(format!("Failed to find texture: \"{name}\"")))?)
    }

    fn get_font(&self, name: &str) -> Result<resources::FontId, resources::Error> {
        Ok(*self
            .fonts_map
            .get(name)
            .ok_or(resources::Error::msg(format!("Failed to find font: \"{name}\"")))?)
    }
}
