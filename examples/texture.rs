use frug::FrugInstance;

extern crate frug;

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    let img_bytes = include_bytes!("uprisen_frog.png");
    let frog_text_idx = frug_instance.load_texture(img_bytes) as u16;

    let img_bytes = include_bytes!("other_frog.png");
    let frog2_text_idx = frug_instance.load_texture(img_bytes) as u16;

    let update_function = move |instance: &mut FrugInstance| {
        instance.clear();
        instance.add_text_rect(0.0, 0.0, 0.5, 0.5, frog_text_idx);
        instance.add_text_rect(-0.5, 0.0, 0.5, 0.5, frog2_text_idx);
        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
