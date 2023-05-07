extern crate frug;

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    let img_bytes = include_bytes!("frog.png");
    let frog_text_idx = frug_instance.load_texture(img_bytes);

    let update_function = move |instance: &mut frug::FrugInstance, _input: &frug::InputHelper| {
        instance.clear();

        instance.add_tex_rect(-0.25, 0.0, 0.5, 0.5, frog_text_idx);

        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
