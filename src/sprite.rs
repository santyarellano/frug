use sdl3::render::{Canvas, Texture};

/// This struct will assume that there is a row per "animation".
/// `frames_in_rows` will contain the number of frames per each row.
pub struct Sprite<'a> {
    pub texture: Texture<'a>,
    pub rows: u32,
    pub frames_in_rows: Vec<u32>,
    pub drawing_rect: sdl3::rect::Rect,
    current_frame: u32,
    current_row: u32,
}

impl<'a> Sprite<'a> {
    /// Creates a new sprite with the given texture, rows, frames in rows, width, and height of texture.
    pub fn new(
        texture: Texture<'a>,
        rows: u32,
        frames_in_rows: Vec<u32>,
        width: u32,
        height: u32,
    ) -> Self {
        Sprite {
            texture,
            rows,
            frames_in_rows,
            drawing_rect: sdl3::rect::Rect::new(0, 0, width, height),
            current_frame: 0,
            current_row: 0,
        }
    }

    /// Sets the animation given a specific row at index 0.
    pub fn start_animation(&mut self, row: u32) {
        self.current_row = row;
        self.current_frame = 0;
        self.drawing_rect
            .set_y(row as i32 * self.drawing_rect.height() as i32);
    }

    /// Updates the sprite's current frame.
    pub fn update(&mut self) {
        self.current_frame += 1;
        if self.current_frame >= self.frames_in_rows[self.current_row as usize] {
            self.current_frame = 0;
        }

        self.drawing_rect
            .set_x(self.current_frame as i32 * self.drawing_rect.width() as i32);
    }
}
