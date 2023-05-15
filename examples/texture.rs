extern crate frug;

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    let img_bytes = include_bytes!("frog.png");
    let frog_text_idx = frug_instance.load_texture(img_bytes);

    let img_bytes = include_bytes!("other_frog.png");
    let frog2_text_idx = frug_instance.load_texture(img_bytes);

    let mut tex_to_use = frog_text_idx;

    let update_function = move |instance: &mut frug::FrugInstance, input: &frug::InputHelper| {
        // Act on input
        // We'll test our input by changing the frog we're using when we press space bar or mouse left click.
        if input.mouse_pressed(frug::MouseButton::Left.into())
            || input.key_pressed(frug::VirtualKeyCode::Space)
        {
            tex_to_use = if tex_to_use == frog_text_idx {
                frog2_text_idx
            } else {
                frog_text_idx
            }
        }

        // We'll also move our camera with left or right input
        let speed = 0.01;
        if input.key_held(frug::VirtualKeyCode::Right) {
            instance.camera.eye.x -= speed;
            instance.camera.target.x -= speed;
        } else if input.key_held(frug::VirtualKeyCode::Left) {
            instance.camera.eye.x += speed;
            instance.camera.target.x += speed;
        }

        // Draw
        instance.clear();
        instance.add_tex_rect(-0.25, 0.0, 0.5, 0.5, tex_to_use, false, false);
        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
