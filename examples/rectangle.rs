use frug::FrugInstance;

extern crate frug;

fn main() {

    let update_loop  = move |instance: &mut FrugInstance| {

        instance.clear_staging_buffers_data();
        instance.add_rectangle(0.0, 0.0, 0.5, 0.5, [0.0, 0.5, 0.5]);
        instance.update_buffers_with_staging();
    };
    
    frug::run("My Window", update_loop);
}