use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::{Canvas, Texture, TextureCreator};
use sdl3::video::{Window, WindowContext};
use sdl3::Sdl;

use crate::sprite::Sprite;

pub struct Instance {
    context: Sdl,
    canvas: Canvas<Window>,
}

impl Instance {
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

    pub fn new_texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.canvas.set_draw_color(color);
        let rect = Rect::new(x, y, width, height);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_image(&mut self, texture: &Texture, x: i32, y: i32, width: u32, height: u32) {
        let rect = Rect::new(x, y, width, height);
        self.canvas
            .copy(texture, None, Some(rect).map(|r| r.into()))
            .unwrap();
    }

    pub fn use_shader(&mut self, _shader_code: &str) {
        // Custom shader implementation goes here
        // SDL3 does not support shaders directly, you might need to use OpenGL or another library
    }

    /// Draws from a texture into a destination rectangle in the canvas.
    pub fn draw(
        &mut self,
        texture: &Texture,
        src_x: i32,
        src_y: i32,
        src_width: u32,
        src_height: u32,
        dest_x: i32,
        dest_y: i32,
        dest_width: u32,
        dest_height: u32,
    ) {
        let src_rect = Rect::new(src_x, src_y, src_width, src_height);
        let dest_rect = Rect::new(dest_x, dest_y, dest_width, dest_height);
        if let Err(e) = self.canvas.copy(texture, src_rect, dest_rect) {
            eprintln!("Error drawing sprite: {}", e);
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, x: i32, y: i32) {
        self.draw(
            &sprite.texture,
            sprite.drawing_rect.x(),
            sprite.drawing_rect.y(),
            sprite.drawing_rect.width(),
            sprite.drawing_rect.width(),
            x,
            y,
            sprite.drawing_rect.width(),
            sprite.drawing_rect.width(),
        );
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        let mut event_pump = self.context.event_pump().unwrap();
        return event_pump.poll_iter().collect::<Vec<_>>();
    }
}
