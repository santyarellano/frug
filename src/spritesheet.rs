use sdl3::render::Texture;

pub struct Spritesheet<'a> {
    texture: Texture<'a>,
    rows: u32,
    columns: u32,
    rect_width: u32,
    rect_height: u32,
}

impl<'a> Spritesheet<'a> {
    pub fn new(
        texture: Texture<'a>,
        rows: u32,
        columns: u32,
        rect_width: u32,
        rect_height: u32,
    ) -> Self {
        let rect_width = width / columns;
        let rect_height = height / rows;

        Spritesheet {
            texture,
            rows,
            columns,
            rect_width,
            rect_height,
        }
    }

    pub fn draw_sprite(&self, index: u32, x: i32, y: i32, width: u32, height: u32) {
        let row = index / self.columns;
        let column = index % self.columns;

        let src_rect = sdl3::rect::Rect::new(
            column as i32 * self.rect_width as i32,
            row as i32 * self.rect_height as i32,
            self.rect_width,
            self.rect_height,
        );

        let dest_rect = sdl3::rect::Rect::new(x, y, width, height);

        self.texture
            .query()
            .with_texture_data(|data| {
                self.texture.with_lock(None, |buffer, _| {
                    for y in 0..height {
                        for x in 0..width {
                            let src_index = (src_rect.y() as usize + y as usize)
                                * data.pitch as usize
                                + (src_rect.x() as usize + x as usize) * 4;
                            let dest_index = (dest_rect.y() as usize + y as usize)
                                * data.pitch as usize
                                + (dest_rect.x() as usize + x as usize) * 4;

                            buffer[dest_index..dest_index + 4]
                                .copy_from_slice(&data.pixels[src_index..src_index + 4]);
                        }
                    }
                })
            })
            .unwrap();
    }
}
