use frug::FrugInstance;

extern crate frug;

fn main() {

    let (mut frug_instance, event_loop) = frug::new("My Window");

    let img_bytes = include_bytes!("uprisen_frog.png");
    frug_instance.load_texture(img_bytes);

    let update_function  = move |instance: &mut FrugInstance| {

        instance.clear_staging_buffers_data();
        instance.add_rectangle(0.0, 0.0, 0.5, 0.5, 0);
        instance.update_buffers_with_staging();
    };
    
    frug_instance.run(event_loop, update_function);
}