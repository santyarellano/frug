use frug::{Color, Event, FrugInstance, Keycode};

fn main() {
    let mut frug_instance = FrugInstance::new("Spritesheet Example", 800, 600);

    'running: loop {
        frug_instance.clear();

        // get input
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

        // Draw a sprite from the spritesheet
        frug_instance.draw_rect(50, 50, 100, 70, Color::RGB(255, 0, 0));

        frug_instance.present();
    }
}
