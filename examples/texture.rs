use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use std::env;
use std::path::Path;

pub fn run(png: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;

    let mut canvas = frug::create_window(&sdl_context);

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png)?;

    frug::draw_textured_rectangle(&mut canvas, &texture, 50, 50, 100, 100);
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        run(Path::new(&args[1]))?;
    }

    Ok(())
}
