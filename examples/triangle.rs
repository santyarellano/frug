extern crate frug;

fn main() {
    let update  = || {
        print!("hello world");
    };

    frug::run("My window", update);
}