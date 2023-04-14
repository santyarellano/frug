use frug::FrugInstance;

extern crate frug;

fn main() {

    // Docs here...
    let mut vertices= [
        frug::Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] },
        frug::Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
        frug::Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
        frug::Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] },
        frug::Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] },
    ];

    let indices: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4
    ];

    let mut col = [0.0, 0.0, 0.0];

    let update_loop  = move |instance: &mut FrugInstance| {
        
        vertices[0].color = [col[0], col[1], col[2]];
        vertices[1].color = [col[1], col[0], col[2]];
        vertices[2].color = [col[1], col[2], col[0]];
        vertices[3].color = [col[2], col[0], col[1]];
        vertices[4].color = [col[1], col[2], col[2]];

        col[0] += 0.001;
        col[1] += 0.002;
        col[2] += 0.003;

        for c in col.iter_mut() {
            if *c >= 1.0 {
                *c -= 1.0;
            }
        }

        instance.clear_staging_buffers_data();
        instance.add_staging_indexed_vertices(&vertices, &indices);
        instance.update_buffers_with_staging();
    };

    frug::run("My Window", update_loop);
}