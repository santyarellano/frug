use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::{Canvas, Texture, TextureCreator};
use sdl3::ttf::{Font, Sdl3TtfContext};
use sdl3::video::{Window, WindowContext};
use sdl3::Sdl;

use crate::sprite::Sprite;
use crate::Vec2d;

/// Frug Instance. Holds the context and the canvas
pub struct Instance {
    context: Sdl,
    canvas: Canvas<Window>,
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

        let canvas = window.into_canvas();

        Instance { context, canvas }
    }

    /// Creates a texture creater which we can use to load textures.
    pub fn new_texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }

    /// Create a ttf context which we can use to load fonts.
    /// Returns an error if the context could not be created.
    pub fn new_ttf_context(&self) -> Result<Sdl3TtfContext, String> {
        sdl3::ttf::init().map_err(|e| e.to_string())
    }

    /// Clears the canvas with a given color
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /// Renders all textures/shapes drawn into the canvas since it was last cleaned.
    pub fn present(&mut self) {
        self.canvas.present();
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

    /// Creates a texture from a given font and a given text.
    /// `font` - The font to use.
    /// `text` - The text to render.
    /// `color` - The color of the text.
    /// `texture_creator` - The texture creator to use.
    pub fn create_text_texture<'a>(
        &mut self,
        font: &Font,
        text: &str,
        color: &Color,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Texture<'a>, String> {
        let surface = match font.render(text).blended(*color) {
            Ok(surface) => surface,
            Err(e) => return Err(format!("Failed to create surface for text texture: {}", e)),
        };

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| format!("Failed to create text texture from surface: {}", e))?;

        Ok(texture)
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

    /// Returns a vector containing all the events captured during the current call of this method.
    pub fn get_events(&mut self) -> Vec<Event> {
        let mut event_pump = self.context.event_pump().unwrap();
        return event_pump.poll_iter().collect::<Vec<_>>();
    }
}
