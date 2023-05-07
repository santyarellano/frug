extern crate frug;

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    let background_color = frug::create_color(0.1, 0.2, 0.3, 1.0);
    frug_instance.set_background_color(background_color);

    let update_function = move |instance: &mut frug::FrugInstance, _input: &frug::InputHelper| {
        instance.clear();
        instance.add_colored_rect(0.0, 0.0, 0.75, 0.5, [0.0, 0.5, 0.5]);
        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
