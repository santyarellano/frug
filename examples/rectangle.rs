use frug::{Color, Event, Instance, Keycode};

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", 800, 600);

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
        frug_instance.draw_rect(50, 50, 100, 70, Color::RGB(255, 0, 0));
        frug_instance.present();
    }
}
