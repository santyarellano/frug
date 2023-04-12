use frug::FrugInstance;

extern crate frug;

fn main() {

    let update_loop  = move |_instance: &mut FrugInstance| {
    };

    frug::run("My Window", update_loop);
}