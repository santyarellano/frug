use frug::{Color, Event, FrugInstance, Keycode};

fn main() {
    let mut frug_instance = FrugInstance::new("Spritesheet Example", 800, 600);

    // load the spritesheet
    let spritesheet = frug_instance.load_image("path/to/spritesheet.png");

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
        frug_instance.clear();
        frug_instance.present();
    }
}
