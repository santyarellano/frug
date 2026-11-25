//! Bitmap fonts: either a horizontal strip of equally sized glyph cells (e.g. a sprite font)
//! or an atlas with an explicit glyph table.

use std::collections::HashMap;
use std::path::Path;

use sdl3::image::LoadSurface;
use sdl3::pixels::PixelFormat;
use sdl3::render::FRect;
use sdl3::surface::Surface;

use crate::{Assets, DrawParams, Instance, TextureId, Vec2d};

/// One glyph inside the font texture.
#[derive(Clone, Copy, Debug)]
pub struct Glyph {
    /// Source rectangle in the texture.
    pub src: FRect,
    /// Horizontal offset applied when drawing.
    pub offset_x: f32,
    /// Vertical offset applied when drawing.
    pub offset_y: f32,
    /// How far the pen moves after this glyph.
    pub advance: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

pub struct BitmapFont {
    pub texture: TextureId,
    glyphs: HashMap<char, Glyph>,
    pub line_height: f32,
    /// Extra space between glyphs.
    pub spacing: f32,
}

impl BitmapFont {
    /// Build a font from an explicit glyph table.
    pub fn new(texture: TextureId, glyphs: HashMap<char, Glyph>, line_height: f32) -> Self {
        Self {
            texture,
            glyphs,
            line_height,
            spacing: 0.0,
        }
    }

    /// Load a strip of `cell_w` x `cell_h` cells, one per character starting at `first`.
    /// With `proportional`, each glyph's width is trimmed to its opaque columns and
    /// `separation` pixels are added between glyphs (like GameMaker's `font_add_sprite`).
    pub fn from_strip<P: AsRef<Path>>(
        assets: &mut Assets,
        path: P,
        first: char,
        cell_w: u32,
        cell_h: u32,
        proportional: bool,
        separation: f32,
    ) -> Result<Self, String> {
        let surface = Surface::from_file(path.as_ref())
            .map_err(|e| format!("{}: {e}", path.as_ref().display()))?
            .convert_format(PixelFormat::RGBA32)
            .map_err(|e| e.to_string())?;
        let (w, h, pitch) = (surface.width(), surface.height(), surface.pitch() as usize);
        let cols = w / cell_w;
        let rows = h / cell_h;
        let mut pixels = Vec::new();
        let mut surface = surface;
        surface.with_lock_mut(|px| pixels.extend_from_slice(px));
        let alpha = |x: u32, y: u32| pixels[y as usize * pitch + x as usize * 4 + 3];

        let mut glyphs = HashMap::new();
        for i in 0..cols * rows {
            let Some(ch) = char::from_u32(first as u32 + i) else {
                continue;
            };
            let cx = (i % cols) * cell_w;
            let cy = (i / cols) * cell_h;
            let mut left = cell_w;
            let mut right = 0;
            for x in 0..cell_w {
                if (0..cell_h).any(|y| alpha(cx + x, cy + y) > 0) {
                    left = left.min(x);
                    right = right.max(x + 1);
                }
            }
            let glyph = if proportional && right > left {
                let gw = (right - left) as f32;
                Glyph {
                    src: FRect::new((cx + left) as f32, cy as f32, gw, cell_h as f32),
                    offset_x: 0.0,
                    offset_y: 0.0,
                    advance: gw + separation,
                }
            } else {
                Glyph {
                    src: FRect::new(cx as f32, cy as f32, cell_w as f32, cell_h as f32),
                    offset_x: 0.0,
                    offset_y: 0.0,
                    advance: if proportional {
                        // Blank cell (space): use a narrower gap.
                        (cell_w as f32 / 2.0).ceil() + separation
                    } else {
                        cell_w as f32 + separation
                    },
                }
            };
            glyphs.insert(ch, glyph);
        }
        let texture = assets.add_surface(&surface, false)?;
        Ok(Self::new(texture, glyphs, cell_h as f32))
    }

    pub fn glyph(&self, ch: char) -> Option<&Glyph> {
        self.glyphs.get(&ch)
    }

    /// Width of one line of text in unscaled pixels.
    pub fn char_width(&self, ch: char) -> f32 {
        self.glyphs.get(&ch).map_or(0.0, |g| g.advance) + self.spacing
    }

    /// Width of the widest line in `text` (unscaled).
    pub fn measure(&self, text: &str) -> f32 {
        text.split('\n')
            .map(|line| line.chars().map(|c| self.char_width(c)).sum::<f32>())
            .fold(0.0, f32::max)
    }

    /// Height of `text` (unscaled).
    pub fn measure_height(&self, text: &str) -> f32 {
        text.split('\n').count() as f32 * self.line_height
    }

    /// Draw a single glyph with its top-left at (x, y). Returns the scaled advance.
    pub fn draw_char(
        &self,
        instance: &mut Instance,
        assets: &Assets,
        ch: char,
        x: f32,
        y: f32,
        params: &DrawParams,
    ) -> f32 {
        let Some(g) = self.glyphs.get(&ch) else {
            return 0.0;
        };
        if ch.is_whitespace() {
            // Whitespace only advances the pen (sprite fonts often mark its width with pixels).
            return (g.advance + self.spacing) * params.scale.x;
        }
        let p = DrawParams {
            src: Some(g.src),
            origin: Vec2d { x: 0.0, y: 0.0 },
            ..*params
        };
        let pos = Vec2d {
            x: x + g.offset_x * params.scale.x,
            y: y + g.offset_y * params.scale.y,
        };
        instance.draw_texture_ex(assets.texture(self.texture), pos, &p);
        (g.advance + self.spacing) * params.scale.x
    }

    /// Draw (possibly multi-line) text aligned around (x, y). Uses scale, alpha, tint and
    /// additive from `params`.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        instance: &mut Instance,
        assets: &Assets,
        text: &str,
        x: f32,
        y: f32,
        halign: HAlign,
        valign: VAlign,
        params: &DrawParams,
    ) {
        let sx = params.scale.x;
        let sy = params.scale.y;
        let total_h = self.measure_height(text) * sy;
        let mut ly = match valign {
            VAlign::Top => y,
            VAlign::Middle => y - total_h / 2.0,
            VAlign::Bottom => y - total_h,
        };
        for line in text.split('\n') {
            let w = line.chars().map(|c| self.char_width(c)).sum::<f32>() * sx;
            let mut lx = match halign {
                HAlign::Left => x,
                HAlign::Center => x - w / 2.0,
                HAlign::Right => x - w,
            };
            for c in line.chars() {
                lx += self.draw_char(instance, assets, c, lx, ly, params);
            }
            ly += self.line_height * sy;
        }
    }
}
