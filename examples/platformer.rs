extern crate frug;

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
        //instance.add_colored_rect(-0.05, 0.05, 0.1, 0.1, [1.0, 1.0, 1.0]);
        instance.add_tex_rect(-0.05, 0.05, 0.1, 0.1, background_tex_idx);

        // present
        instance.update_buffers();
        // ======= RENDER ======
    };

    frug_instance.run(event_loop, update_function);
}
