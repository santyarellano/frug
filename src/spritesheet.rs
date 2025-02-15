use sdl3::render::Texture;

pub struct Spritesheet<'a> {
    texture: Texture<'a>,
    rows: u32,
    columns: u32,
    rect_width: u32,
    rect_height: u32,
}
