use std::time::Duration;

use frug::{self, Color, Event, Keycode};

fn main() {
    let graphics_context = sdl3::init().unwrap();
    let mut canvas = frug::create_window(&graphics_context);
    let background_color = Color::RGB(50, 50, 50);

    canvas.set_draw_color(background_color);
    canvas.clear();
    canvas.present();

    let mut event_pump = graphics_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(background_color);
        canvas.clear();

        // input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // ** Game loop here **
        frug::draw_rectangle(&mut canvas, Color::RGB(50, 50, 255), 50, 50, 100, 100);
        // ** End of game loop **

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
