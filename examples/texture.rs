use frug::{Color, Event, Instance, Keycode, LoadTexture};

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", 800, 600);
    let background_color = Color::RGB(100, 100, 150);

    // load the spritesheet
    let texture_creator = frug_instance.new_texture_creator();
    let texture = match texture_creator.load_texture("examples/frog.png") {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Failed to load texture: {}", e);
            return;
        }
    };

    'running: loop {
        // Input
        for event in frug_instance.get_events() {
            match event {
                // Quit the application
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Render
        frug_instance.clear(background_color);
        frug_instance.draw_image(&texture, 200, 200, 200, 200);
        frug_instance.present();
    }
}
