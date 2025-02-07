use frug::{self, Color, Event, Keycode};
use sdl3::image::LoadTexture;
use std::time::Duration;

fn main() {
    let graphics_context = sdl3::init().unwrap();

    let mut canvas = frug::create_window(&graphics_context);

    let texture_creator = canvas.texture_creator();
    let texture = match texture_creator.load_texture("examples/other_frog.png") {
        Ok(texture) => texture,
        Err(e) => {
            eprintln!("Failed to load texture: {}", e);
            return;
        }
    };

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
        //frug::draw_rectangle(&mut canvas, Color::RGB(150, 150, 150), 50, 50, 100, 100);
        frug::draw_textured_rectangle(&mut canvas, &texture, 50, 50, 100, 100);
        // ** End of game loop **

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
