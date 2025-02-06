//! FRUG is intended to provide a similar abstraction layer over graphics programming as to how SDL does for C++, meaning that it should provide developers enough control and flexibility to implement their own architectures & design patterns, yet simplifying the process of working with graphics so developers won't have to worry about implementing all the repetitive tasks related to getting things to the screen.
//!
//! Please see [the documentation](https://santyarellano.github.io/frug_book/) for more information.
//!
//! I'M MIGRATING THIS WHOLE THING FROM WGPU TO SDL3. PLEASE HOLD ON!!!

pub use sdl3::event::Event;
pub use sdl3::keyboard::Keycode;
pub use sdl3::pixels::Color;

use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::Sdl;

/// Returns a canvas where we can draw.
pub fn create_window(sdl_context: &Sdl) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("FRUG Window", 800, 600)
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
