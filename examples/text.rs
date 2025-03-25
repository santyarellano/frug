use frug::{create_text_texture, Color, Event, Instance, Keycode, TextureQuery, Vec2d};

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", 800, 600);
    let background_color = Color::RGB(70, 70, 100);

    // load the font and create a texture creator
    let texture_creator = frug_instance.new_texture_creator();
    let ttf_context = frug_instance.new_ttf_context().unwrap();

    // text settings
    let font_scale = 7.0;
    let font_size = 8.0 * font_scale;
    let text = "FRUG!";
    let text_color = Color::RGB(100, 255, 100);

    // create the font
    let font = match ttf_context.load_font("examples/PressStart2P-Regular.ttf", font_size) {
        Ok(font) => font,
        Err(e) => {
            eprintln!("Failed to load font: {}", e);
            return;
        }
    };

    // create the text texture
    let text_texture = match create_text_texture(&font, text, &text_color, &texture_creator) {
        Ok(texture) => texture,
        Err(e) => {
            eprintln!("Failed to create text texture: {}", e);
            return;
        }
    };

    // get the dimensions of the text texture
    let TextureQuery { width, height, .. } = text_texture.query();

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
        frug_instance.draw_full_texture(
            &text_texture,
            &Vec2d { x: 50, y: 200 },
            &Vec2d {
                x: width,
                y: height,
            },
        );
        frug_instance.present();
    }
}
