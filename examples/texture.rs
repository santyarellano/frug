use frug::FrugInstance;

extern crate frug;

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    let img_bytes = include_bytes!("frog.png");
    let frog_text_idx = frug_instance.load_texture(img_bytes);

    let img_bytes = include_bytes!("other_frog.png");
    let frog2_text_idx = frug_instance.load_texture(img_bytes);

    let mut xpos = 0.0;

    let update_function = move |instance: &mut FrugInstance| {
        instance.clear();
        instance.add_text_rect(xpos - 0.75, 0.0, 0.5, 0.5, frog2_text_idx);
        instance.add_text_rect(xpos - 0.0, 0.0, 0.5, 0.5, frog_text_idx);
        instance.update_buffers();

        xpos += 0.001;
    };

    frug_instance.run(event_loop, update_function);
}
