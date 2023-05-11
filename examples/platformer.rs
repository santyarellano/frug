extern crate frug;

/// This function helps us draw the same texture for our background on repeat.
fn draw_repeat_background(instance: &mut frug::FrugInstance, tex_idx: usize, rows: u16, cols: u16) {
    let tile_w: f32 = 2.0 / cols as f32;
    let tile_h: f32 = 2.0 / rows as f32;
    for i in 0..rows {
        for j in 0..cols {
            instance.add_tex_rect(
                tile_w * j as f32 - 1.0,
                tile_h * -(i as f32) + 1.0,
                tile_w,
                tile_h,
                tex_idx,
            );
        }
    }
}

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    // ======= WINDOW SETUP ======
    frug_instance.set_window_size(800.0, 800.0);
    // ======= WINDOW SETUP ======

    // ======= LOAD ASSETS ======
    // background
    let img_bytes = include_bytes!("platformer_imgs/Purple.png");
    let background_tex_idx = frug_instance.load_texture(img_bytes);

    // land
    let img_bytes = include_bytes!("platformer_imgs/land.png");
    let land_tex_idx = frug_instance.load_texture(img_bytes);

    // frog
    // ======= LOAD ASSETS ======

    let update_function = move |instance: &mut frug::FrugInstance, input: &frug::InputHelper| {
        // ======= RENDER ======
        instance.clear();
        // background
        draw_repeat_background(instance, background_tex_idx, 6, 6);

        // present
        instance.update_buffers();
        // ======= RENDER ======
    };

    frug_instance.run(event_loop, update_function);
}
