use std::collections::HashMap;
use std::path::Path;

use sdl3::image::LoadSurface;
use sdl3::pixels::PixelFormat;
use sdl3::render::{ScaleMode, Texture, TextureCreator};
use sdl3::surface::Surface;
use sdl3::video::WindowContext;

/// Handle to a texture owned by [`Assets`]. Cheap to copy and store in game state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(usize);

struct Entry {
    texture: Texture<'static>,
    /// Optional white silhouette of the same image (alpha preserved), for hit flashes.
    flash: Option<Texture<'static>>,
    width: u32,
    height: u32,
}

/// Owns textures so game code can hold plain [`TextureId`]s instead of borrowed textures.
pub struct Assets {
    creator: &'static TextureCreator<WindowContext>,
    entries: Vec<Entry>,
    by_path: HashMap<String, TextureId>,
}

impl Assets {
    pub fn new(creator: &'static TextureCreator<WindowContext>) -> Self {
        Self {
            creator,
            entries: Vec::new(),
            by_path: HashMap::new(),
        }
    }

    /// Load an image file (PNG, JPG, BMP, ...) with nearest-neighbour filtering.
    /// Loading the same path twice returns the same id.
    pub fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> Result<TextureId, String> {
        self.load(path, false)
    }

    /// Like [`Assets::load_texture`] but also builds a white silhouette used by
    /// [`crate::DrawParams::flash`].
    pub fn load_texture_with_flash<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<TextureId, String> {
        self.load(path, true)
    }

    fn load<P: AsRef<Path>>(&mut self, path: P, flash: bool) -> Result<TextureId, String> {
        let key = path.as_ref().to_string_lossy().into_owned();
        if let Some(id) = self.by_path.get(&key) {
            if flash && self.entries[id.0].flash.is_none() {
                let surface = load_rgba(path.as_ref())?;
                self.entries[id.0].flash = Some(self.white_texture(&surface)?);
            }
            return Ok(*id);
        }
        let surface = load_rgba(path.as_ref())?;
        let id = self.add_surface(&surface, flash)?;
        self.by_path.insert(key, id);
        Ok(id)
    }

    /// Create a texture from an SDL surface.
    pub fn add_surface(&mut self, surface: &Surface, flash: bool) -> Result<TextureId, String> {
        let mut texture = self
            .creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())?;
        texture.set_scale_mode(ScaleMode::Nearest);
        let flash = if flash {
            let rgba = surface
                .convert_format(PixelFormat::RGBA32)
                .map_err(|e| e.to_string())?;
            Some(self.white_texture(&rgba)?)
        } else {
            None
        };
        let id = TextureId(self.entries.len());
        self.entries.push(Entry {
            texture,
            flash,
            width: surface.width(),
            height: surface.height(),
        });
        Ok(id)
    }

    /// Take ownership of a texture created elsewhere (e.g. a text texture).
    pub fn add_texture(&mut self, texture: Texture<'static>) -> TextureId {
        let q = texture.query();
        let id = TextureId(self.entries.len());
        self.entries.push(Entry {
            texture,
            flash: None,
            width: q.width,
            height: q.height,
        });
        id
    }

    fn white_texture(&self, rgba: &Surface) -> Result<Texture<'static>, String> {
        let mut white = rgba
            .convert_format(PixelFormat::RGBA32)
            .map_err(|e| e.to_string())?;
        white.with_lock_mut(|px| {
            for p in px.chunks_exact_mut(4) {
                p[0] = 255;
                p[1] = 255;
                p[2] = 255;
            }
        });
        let mut tex = self
            .creator
            .create_texture_from_surface(&white)
            .map_err(|e| e.to_string())?;
        tex.set_scale_mode(ScaleMode::Nearest);
        Ok(tex)
    }

    pub fn texture(&self, id: TextureId) -> &Texture<'static> {
        &self.entries[id.0].texture
    }

    pub fn texture_mut(&mut self, id: TextureId) -> &mut Texture<'static> {
        &mut self.entries[id.0].texture
    }

    /// The white silhouette, if the texture was loaded with one.
    pub fn flash_texture(&self, id: TextureId) -> Option<&Texture<'static>> {
        self.entries[id.0].flash.as_ref()
    }

    /// (width, height) in pixels.
    pub fn size(&self, id: TextureId) -> (u32, u32) {
        let e = &self.entries[id.0];
        (e.width, e.height)
    }

    pub fn creator(&self) -> &'static TextureCreator<WindowContext> {
        self.creator
    }
}

fn load_rgba(path: &Path) -> Result<Surface<'static>, String> {
    let surface = Surface::from_file(path).map_err(|e| format!("{}: {e}", path.display()))?;
    surface
        .convert_format(PixelFormat::RGBA32)
        .map_err(|e| e.to_string())
}
