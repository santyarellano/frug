extern crate frug;

fn main() {
    let update  = || {
        print!("hello world");
    };

    frug::init("My window", update);
}