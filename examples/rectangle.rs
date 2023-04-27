use frug::FrugInstance;

extern crate frug;

fn main() {
    let (frug_instance, event_loop) = frug::new("My Window");

    let mut x = 0.0;

    let update_function = move |instance: &mut FrugInstance, _input: &frug::InputHelper| {
        instance.clear();
        instance.add_colored_rect(x, 0.0, 0.5, 0.5, [0.0, 0.5, 0.5]);
        instance.update_buffers();

        x += 0.001;
    };

    frug_instance.run(event_loop, update_function);
}
