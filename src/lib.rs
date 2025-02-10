//! FRUG is intended to provide a similar abstraction layer over graphics programming as to how SDL does for C++, meaning that it should provide developers enough control and flexibility to implement their own architectures & design patterns, yet simplifying the process of working with graphics so developers won't have to worry about implementing all the repetitive tasks related to getting things to the screen.
//!
//! Please see [the documentation](https://santyarellano.github.io/frug_book/) for more information.
//!
//! I'M MIGRATING THIS WHOLE THING FROM WGPU TO SDL3. PLEASE HOLD ON!!!

pub use sdl3::event::Event;
pub use sdl3::image::LoadTexture;
pub use sdl3::keyboard::Keycode;
pub use sdl3::pixels::Color;
pub use sdl3::surface::Surface;

use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::render::Texture;
use sdl3::video::Window;
use sdl3::{Error, Sdl};

/// Inits frug and returns the context.
pub fn init() -> Result<Sdl, Error> {
    let context = sdl3::init();
    return context;
}

/// Returns a canvas where we can draw.
pub fn create_window(sdl_context: &Sdl, width: u32, height: u32) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("FRUG Window", width, height)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas();
    return canvas;
}

/// Draws a rectangle on the given canvas using the given color, position, and dimensions.
pub fn draw_rectangle(
    canvas: &mut Canvas<Window>,
    color: Color,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) {
    canvas.set_draw_color(color);
    let _ = canvas.fill_rect(Rect::new(x, y, width, height));
}

/// Draws a rectangle with a given texture on the given canvas at the specified position and dimensions.
pub fn draw_textured_rectangle(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) {
    let rect = Rect::new(x, y, width, height);
    let _ = canvas.copy(&texture, None, rect);
}
