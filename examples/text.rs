use frug::{Color, Event, Instance, Keycode};

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", 800, 600);
    let background_color = Color::RGB(100, 100, 150);

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
        frug_instance.present();
    }
}
