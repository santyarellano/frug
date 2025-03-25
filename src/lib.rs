//! FRUG is intended to provide a similar abstraction layer over graphics programming as to how SDL does for C++, meaning that it should provide developers enough control and flexibility to implement their own architectures & design patterns, yet simplifying the process of working with graphics so developers won't have to worry about implementing all the repetitive tasks related to getting things to the screen.
//!
//! Please see [the documentation](https://santyarellano.github.io/frug_book/) for more information.
//!
//! I'M MIGRATING THIS WHOLE THING FROM WGPU TO SDL3. PLEASE HOLD ON!!!

mod instance;
mod sprite;
mod vectors;

pub use instance::Instance;
pub use sdl3;
pub use sdl3::event::Event;
pub use sdl3::event::EventPollIterator;
pub use sdl3::image::LoadTexture;
pub use sdl3::keyboard::Keycode;
pub use sdl3::pixels::Color;
pub use sdl3::rect::Rect;
pub use sdl3::render;
pub use sdl3::render::ScaleMode;
pub use sdl3::render::TextureQuery;
pub use sdl3::ttf;
pub use sdl3::video;
pub use sprite::Sprite;
pub use vectors::Vec2d;

/// Creates a texture from a given font and a given text.
/// `font` - The font to use.
/// `text` - The text to render.
/// `color` - The color of the text.
/// `texture_creator` - The texture creator to use.
pub fn create_text_texture<'a>(
    font: &ttf::Font,
    text: &str,
    color: &Color,
    texture_creator: &'a render::TextureCreator<video::WindowContext>,
) -> Result<render::Texture<'a>, String> {
    let surface = match font.render(text).blended(*color) {
        Ok(surface) => surface,
        Err(e) => return Err(format!("Failed to create surface for text texture: {}", e)),
    };

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| format!("Failed to create text texture from surface: {}", e))?;

    Ok(texture)
}
