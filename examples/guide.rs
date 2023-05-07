extern crate frug;

fn main() {
    let (frug_instance, event_loop) = frug::new("My Window");

    let red = [1.0, 0.0, 0.0];
    let green = [0.0, 1.0, 0.0];
    let blue = [0.0, 0.0, 1.0];

    let vertices = [
        frug::Vertex {
            // Vertex 0
            position: [0.0, 0.5, 0.0],
            color: red,
            ..Default::default()
        },
        frug::Vertex {
            // Vertex 1
            position: [-0.5, -0.5, 0.0],
            color: green,
            ..Default::default()
        },
        frug::Vertex {
            // Vertex 2
            position: [0.5, -0.5, 0.0],
            color: blue,
            ..Default::default()
        },
    ];

    let indices = [0, 1, 2];

    let update_function = move |instance: &mut frug::FrugInstance, _input: &frug::InputHelper| {
        // Rendering
        instance.clear();
        instance.add_colored_vertices(&vertices, &indices);
        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
