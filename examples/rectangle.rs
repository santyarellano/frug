use frug::{Color, Event, Instance, Keycode, Vec2d};

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", 800, 600);

    let rect_pos = Vec2d { x: 50, y: 50 };
    let rect_size = Vec2d { x: 100, y: 70 };

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
        frug_instance.draw_rect(&rect_pos, &rect_size, Color::RGB(255, 0, 0));
        frug_instance.present();
    }
}
