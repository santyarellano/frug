use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::ClippingRect;
use sdl3::render::{BlendMode, Canvas, FPoint, FRect, ScaleMode, Texture, TextureCreator};
use sdl3::sys;
use sdl3::ttf::Sdl3TtfContext;
use sdl3::video::{Window, WindowContext};
use sdl3::{EventPump, Sdl};

use crate::sprite::Sprite;
use crate::Vec2d;

/// Optional parameters for [`Instance::draw_texture_ex`].
#[derive(Clone, Copy, Debug)]
pub struct DrawParams {
    /// Source rectangle inside the texture (x, y, w, h). `None` = whole texture.
    pub src: Option<FRect>,
    /// Pivot inside the source rect, in source pixels. The pivot lands on `pos`.
    pub origin: Vec2d<f32>,
    /// Scale; negative values flip around the origin.
    pub scale: Vec2d<f32>,
    /// Rotation in degrees, clockwise, around the origin.
    pub angle: f64,
    /// 0.0 - 1.0
    pub alpha: f32,
    /// Multiplied with the texture colour.
    pub tint: Color,
    /// Additive blending instead of alpha blending.
    pub additive: bool,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            src: None,
            origin: Vec2d { x: 0.0, y: 0.0 },
            scale: Vec2d { x: 1.0, y: 1.0 },
            angle: 0.0,
            alpha: 1.0,
            tint: Color::RGB(255, 255, 255),
            additive: false,
        }
    }
}

struct Logical {
    width: u32,
    height: u32,
    target: Texture<'static>,
    integer_scale: bool,
}

/// Frug Instance. Holds the context and the canvas
pub struct Instance {
    context: Sdl,
    canvas: Canvas<Window>,
    creator: &'static TextureCreator<WindowContext>,
    event_pump: Option<EventPump>,
    logical: Option<Logical>,
    /// `true` while drawing to the window after the logical image has been blitted.
    in_gui: bool,
    /// World-space camera: subtracted from positions while drawing the world (not the GUI).
    pub camera: Vec2d<f32>,
    letterbox_color: Color,
}

impl Instance {
    /// Creates a new frug instance and a window (given a title, width, and height)
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let context = sdl3::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas();
        canvas.set_blend_mode(BlendMode::Blend);
        // The texture creator is leaked so that textures can be `Texture<'static>` and be
        // stored freely in game state. There is one per window, so this is not a real leak.
        let creator: &'static TextureCreator<WindowContext> =
            Box::leak(Box::new(canvas.texture_creator()));

        Instance {
            context,
            canvas,
            creator,
            event_pump: None,
            logical: None,
            in_gui: false,
            camera: Vec2d { x: 0.0, y: 0.0 },
            letterbox_color: Color::RGB(0, 0, 0),
        }
    }

    /// The SDL context, for subsystems frug does not wrap (gamepad, audio, ...).
    pub fn sdl(&self) -> &Sdl {
        &self.context
    }

    /// The underlying SDL canvas, for anything frug does not wrap.
    pub fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }

    /// Creates a texture creater which we can use to load textures.
    pub fn new_texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }

    /// A texture creator that lives for the whole program; textures it creates are `'static`.
    pub fn texture_creator(&self) -> &'static TextureCreator<WindowContext> {
        self.creator
    }

    /// Create a ttf context which we can use to load fonts.
    /// Returns an error if the context could not be created.
    pub fn new_ttf_context(&self) -> Result<Sdl3TtfContext, String> {
        sdl3::ttf::init().map_err(|e| e.to_string())
    }

    /// Enable or disable vsync.
    pub fn set_vsync(&mut self, on: bool) {
        unsafe {
            sys::render::SDL_SetRenderVSync(self.canvas.raw(), if on { 1 } else { 0 });
        }
    }

    /// Render the world into a `width` x `height` image which is then scaled to the window,
    /// keeping its aspect ratio (letterboxed). With `integer_scale` only whole multiples are
    /// used, which keeps pixel art crisp. Anything drawn after [`Instance::begin_gui`] is
    /// drawn in window pixels on top.
    ///
    /// World drawing is snapped to whole logical pixels, and the camera's fractional part is
    /// applied when the image is scaled up, so scrolling stays smooth (one window pixel at a
    /// time) while sprites stay pixel-perfect.
    pub fn set_logical_size(&mut self, width: u32, height: u32, integer_scale: bool) {
        // One extra row and column to shift into view for sub-pixel camera offsets.
        let mut target = self
            .creator
            .create_texture_target(None, width + 1, height + 1)
            .expect("failed to create logical render target");
        target.set_scale_mode(ScaleMode::Nearest);
        self.logical = Some(Logical {
            width,
            height,
            target,
            integer_scale,
        });
    }

    /// Size of the world drawing surface (the logical size, or the window size).
    pub fn logical_size(&self) -> (u32, u32) {
        match &self.logical {
            Some(l) => (l.width, l.height),
            None => self.window_size(),
        }
    }

    /// Size of the window's drawable area in pixels (the GUI coordinate space).
    /// Unaffected by the current render target.
    pub fn window_size(&self) -> (u32, u32) {
        self.canvas.window().size_in_pixels()
    }

    /// Colour of the bars around the logical image.
    pub fn set_letterbox_color(&mut self, color: Color) {
        self.letterbox_color = color;
    }

    /// Where the logical image lands in the window: (x, y, scale).
    pub fn logical_viewport(&self) -> (f32, f32, f32) {
        let Some(l) = &self.logical else {
            return (0.0, 0.0, 1.0);
        };
        let (ww, wh) = self.window_size();
        let mut s = (ww as f32 / l.width as f32).min(wh as f32 / l.height as f32);
        if l.integer_scale && s >= 1.0 {
            s = s.floor();
        }
        let w = l.width as f32 * s;
        let h = l.height as f32 * s;
        ((ww as f32 - w) / 2.0, (wh as f32 - h) / 2.0, s)
    }

    /// Clears the canvas with a given color. Starts a new frame: drawing goes to the world
    /// (the logical image if one is set) and the camera is applied.
    pub fn clear(&mut self, color: Color) {
        self.in_gui = false;
        if let Some(l) = &self.logical {
            unsafe {
                sys::render::SDL_SetRenderTarget(self.canvas.raw(), l.target.raw());
            }
        }
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /// Finish drawing the world and switch to window-space drawing (HUD, overlays).
    /// The camera is not applied in GUI mode. Called automatically by `present`.
    pub fn begin_gui(&mut self) {
        if self.in_gui {
            return;
        }
        self.in_gui = true;
        if self.logical.is_none() {
            return;
        }
        unsafe {
            sys::render::SDL_SetRenderTarget(self.canvas.raw(), std::ptr::null_mut());
        }
        let (x, y, s) = self.logical_viewport();
        self.canvas.set_draw_color(self.letterbox_color);
        self.canvas.clear();
        let l = self.logical.as_ref().unwrap();
        let raw = l.target.raw();
        unsafe {
            sys::render::SDL_SetTextureColorMod(raw, 255, 255, 255);
            sys::render::SDL_SetTextureAlphaModFloat(raw, 1.0);
            sys::render::SDL_SetTextureBlendMode(raw, sys::blendmode::SDL_BLENDMODE_NONE);
        }
        // Sub-pixel camera: shift the image by the camera's fractional part, in whole
        // window pixels, and clip to the logical viewport.
        let fx = ((self.camera.x - self.camera.x.floor()) * s).round();
        let fy = ((self.camera.y - self.camera.y.floor()) * s).round();
        let (vw, vh) = (l.width as f32 * s, l.height as f32 * s);
        self.canvas.set_clip_rect(Rect::new(
            x.round() as i32,
            y.round() as i32,
            vw.round() as u32,
            vh.round() as u32,
        ));
        let dst = FRect::new(x - fx, y - fy, (l.width + 1) as f32 * s, (l.height + 1) as f32 * s);
        if let Err(e) = self.canvas.copy(&l.target, None, dst) {
            eprintln!("Error presenting logical image: {e}");
        }
        self.canvas.set_clip_rect(ClippingRect::None);
    }

    /// `true` between `begin_gui` and the next `clear`.
    pub fn in_gui(&self) -> bool {
        self.in_gui
    }

    /// Renders all textures/shapes drawn into the canvas since it was last cleaned.
    pub fn present(&mut self) {
        self.begin_gui();
        self.canvas.present();
    }

    fn view_offset(&self) -> (f32, f32) {
        if self.in_gui {
            (0.0, 0.0)
        } else if self.logical.is_some() {
            // The fractional part is applied when the logical image is scaled up.
            (self.camera.x.floor(), self.camera.y.floor())
        } else {
            (self.camera.x, self.camera.y)
        }
    }

    /// Snap to whole logical pixels while drawing the low-resolution world, so pixel art
    /// never lands between pixels (which drops or doubles rows and columns).
    fn snap(&self, v: f32) -> f32 {
        if self.logical.is_some() && !self.in_gui {
            v.round()
        } else {
            v
        }
    }

    /// Draws a rectangle given the position, size, and color
    /// `pos`   - Defines the position of the rectangle
    /// `size`  - Defines the dimensions of the rectangle (x = width, y = height)
    /// `color` - Color
    pub fn draw_rect(&mut self, pos: &Vec2d<i32>, size: &Vec2d<u32>, color: Color) {
        self.canvas.set_draw_color(color);
        let rect = Rect::new(pos.x, pos.y, size.x, size.y);
        self.canvas.fill_rect(rect).unwrap();
    }

    /// Filled rectangle with float coordinates. Respects the camera and colour alpha.
    pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let (cx, cy) = self.view_offset();
        self.canvas.set_draw_color(color);
        let (x0, y0) = (self.snap(x - cx), self.snap(y - cy));
        let (x1, y1) = (self.snap(x - cx + w), self.snap(y - cy + h));
        let _ = self.canvas.fill_rect(FRect::new(x0, y0, x1 - x0, y1 - y0));
    }

    /// Rectangle outline (1px) with float coordinates. Respects the camera.
    pub fn stroke_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let (cx, cy) = self.view_offset();
        self.canvas.set_draw_color(color);
        let _ = self.canvas.draw_rect(FRect::new(x - cx, y - cy, w, h));
    }

    /// Line with float coordinates. Respects the camera.
    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
        let (cx, cy) = self.view_offset();
        self.canvas.set_draw_color(color);
        let _ = self
            .canvas
            .draw_line(FPoint::new(x1 - cx, y1 - cy), FPoint::new(x2 - cx, y2 - cy));
    }

    /// Draws a texture
    /// `texture`   - Defines the texture to use.
    /// `pos`       - Defines the position of the image to draw.
    /// `size`      - Defines the dimensions of the image rectangle (x = width, y = height)
    pub fn draw_full_texture(&mut self, texture: &Texture, pos: &Vec2d<i32>, size: &Vec2d<u32>) {
        let rect = Rect::new(pos.x, pos.y, size.x, size.y);
        self.canvas
            .copy(texture, None, Some(rect).map(|r| r.into()))
            .unwrap();
    }

    /// Draws from a texture into a destination rectangle in the canvas.
    /// `texture` - Defines the texture to use.
    /// `src_pos` - Defines the starting point of the rectangular section of the texture to draw onto the canvas.
    /// `src_dimensions` - Defines the dimensions of the rectangular section of the texture to draw onto the canvas.
    /// `dest_pos` - Defines the starting point of the rectangular section of the canvas where the texture will be drawn.
    /// `dest_dimensions` - Defines the dimensions of the rectangular section of the canvas where the texture will be drawn.
    pub fn draw_texture(
        &mut self,
        texture: &Texture,
        src_pos: &Vec2d<i32>,
        src_dimensions: &Vec2d<u32>,
        dest_pos: &Vec2d<i32>,
        dest_dimensions: &Vec2d<u32>,
    ) {
        let src_rect = Rect::new(src_pos.x, src_pos.y, src_dimensions.x, src_dimensions.y);
        let dest_rect = Rect::new(dest_pos.x, dest_pos.y, dest_dimensions.x, dest_dimensions.y);
        if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
            eprintln!("Error drawing sprite: {}", e);
        }
    }

    /// Draws a texture with float position, origin, scale, flip, rotation, alpha, tint and
    /// blend mode. Respects the camera while drawing the world.
    pub fn draw_texture_ex(&mut self, texture: &Texture, pos: Vec2d<f32>, p: &DrawParams) {
        let src = p.src.unwrap_or_else(|| {
            let q = texture.query();
            FRect::new(0.0, 0.0, q.width as f32, q.height as f32)
        });
        let (cx, cy) = self.view_offset();
        let sx = p.scale.x.abs();
        let sy = p.scale.y.abs();
        // Pivot measured from the destination rect's top-left, after flipping.
        let px = if p.scale.x < 0.0 { src.w - p.origin.x } else { p.origin.x } * sx;
        let py = if p.scale.y < 0.0 { src.h - p.origin.y } else { p.origin.y } * sy;
        let (x0, y0) = (self.snap(pos.x - cx - px), self.snap(pos.y - cy - py));
        let (x1, y1) = (
            self.snap(pos.x - cx - px + src.w * sx),
            self.snap(pos.y - cy - py + src.h * sy),
        );
        let dst = FRect::new(x0, y0, x1 - x0, y1 - y0);

        let raw = texture.raw();
        unsafe {
            sys::render::SDL_SetTextureColorMod(raw, p.tint.r, p.tint.g, p.tint.b);
            sys::render::SDL_SetTextureAlphaModFloat(raw, p.alpha.clamp(0.0, 1.0));
            sys::render::SDL_SetTextureBlendMode(
                raw,
                if p.additive {
                    sys::blendmode::SDL_BLENDMODE_ADD
                } else {
                    sys::blendmode::SDL_BLENDMODE_BLEND
                },
            );
        }
        let result = if p.angle == 0.0 && p.scale.x >= 0.0 && p.scale.y >= 0.0 {
            self.canvas.copy(texture, src, dst)
        } else {
            self.canvas.copy_ex(
                texture,
                src,
                dst,
                p.angle,
                FPoint::new(px, py),
                p.scale.x < 0.0,
                p.scale.y < 0.0,
            )
        };
        if let Err(e) = result {
            eprintln!("Error drawing texture: {e}");
        }
    }

    /// Draws a nine-slice: the `border` pixels at each edge of `src` keep their size and the
    /// middle stretches to fill the `w` x `h` rectangle at (`x`, `y`). Only alpha, tint and
    /// additive are taken from `p`.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_nine_slice(
        &mut self,
        texture: &Texture,
        src: FRect,
        border: f32,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        p: &DrawParams,
    ) {
        // Snap the outer rectangle once so the slices tile without gaps.
        let (x, y) = (x.round(), y.round());
        let (w, h) = (w.round(), h.round());
        let b = border;
        let cols = [
            (src.x, b, x, b),
            (src.x + b, src.w - 2.0 * b, x + b, (w - 2.0 * b).max(0.0)),
            (src.x + src.w - b, b, x + w - b, b),
        ];
        let rows = [
            (src.y, b, y, b),
            (src.y + b, src.h - 2.0 * b, y + b, (h - 2.0 * b).max(0.0)),
            (src.y + src.h - b, b, y + h - b, b),
        ];
        for (sy, sh, dy, dh) in rows {
            for (sx, sw, dx, dw) in cols {
                if sw <= 0.0 || sh <= 0.0 || dw <= 0.0 || dh <= 0.0 {
                    continue;
                }
                let params = DrawParams {
                    src: Some(FRect::new(sx, sy, sw, sh)),
                    origin: Vec2d { x: 0.0, y: 0.0 },
                    scale: Vec2d {
                        x: dw / sw,
                        y: dh / sh,
                    },
                    angle: 0.0,
                    ..*p
                };
                self.draw_texture_ex(texture, Vec2d { x: dx, y: dy }, &params);
            }
        }
    }

    /// Draws a given sprite in its current frame.
    /// `sprite`    - The sprite object to use.
    /// `position`  - Defines the position on the canvas where we want to draw the sprite.
    /// `scale`     - Defines the scale of the sprite to draw.
    pub fn draw_sprite(&mut self, sprite: &Sprite, position: &Vec2d<i32>, scale: &Vec2d<u32>) {
        self.draw_texture(
            &sprite.texture,
            &Vec2d {
                x: sprite.drawing_rect.x,
                y: sprite.drawing_rect.y,
            },
            &Vec2d {
                x: sprite.drawing_rect.width(),
                y: sprite.drawing_rect.height(),
            },
            position,
            &Vec2d {
                x: sprite.drawing_rect.width() * scale.x,
                y: sprite.drawing_rect.height() * scale.y,
            },
        );
    }

    /// Save what has been drawn to the current render target so far as a PNG.
    /// Call it after drawing and before `present` (e.g. at the end of `Game::draw`).
    pub fn save_screenshot<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        use sdl3::image::SaveSurface;
        let surface = self.canvas.read_pixels(None).map_err(|e| e.to_string())?;
        surface.save(path).map_err(|e| e.to_string())
    }

    /// Returns a vector containing all the events captured during the current call of this method.
    pub fn get_events(&mut self) -> Vec<Event> {
        if self.event_pump.is_none() {
            self.event_pump = Some(self.context.event_pump().unwrap());
        }
        self.event_pump.as_mut().unwrap().poll_iter().collect()
    }
}
