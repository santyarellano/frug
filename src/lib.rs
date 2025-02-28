//! FRUG is intended to provide a similar abstraction layer over graphics programming as to how SDL does for C++, meaning that it should provide developers enough control and flexibility to implement their own architectures & design patterns, yet simplifying the process of working with graphics so developers won't have to worry about implementing all the repetitive tasks related to getting things to the screen.
//!
//! Please see [the documentation](https://santyarellano.github.io/frug_book/) for more information.
//!
//! I'M MIGRATING THIS WHOLE THING FROM WGPU TO SDL3. PLEASE HOLD ON!!!

mod instance;
mod sprite;
mod vectors;

pub use instance::Instance;
pub use sdl3::event::Event;
pub use sdl3::event::EventPollIterator;
pub use sdl3::image::LoadTexture;
pub use sdl3::keyboard::Keycode;
pub use sdl3::pixels::Color;
pub use sdl3::rect::Rect;
pub use sprite::Sprite;
pub use vectors::Vec2d;
