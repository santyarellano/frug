extern crate frug;

// - - - - - TEST! - - - - -
// We should remove this in the future so we can create these in frug usage.
const VERTICES: &[frug::Vertex] = &[
    frug::Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] },
    frug::Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
    frug::Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
    frug::Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] },
    frug::Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4
];
// - - - - - TEST! - - - - -

fn main() {
    let mut cnt = 0;

    let update_loop  = move || {
        // Your update code here
        println!("{cnt}");
        cnt += 1;
    };

    frug::run("My window", update_loop);
}