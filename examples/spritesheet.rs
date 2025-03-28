use frug::{Color, Event, Instance, Keycode, LoadTexture, ScaleMode, Sprite, Vec2d};

#[derive(PartialEq, Clone, Copy)]
enum Animation {
    Idle,
    Walk,
}

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", 800, 600);
    let texture_creator = frug_instance.new_texture_creator();
    let background_color = Color::RGB(100, 100, 150);

    // load the spritesheet
    texture_creator.default_pixel_format();
    let mut texture = match texture_creator.load_texture("examples/platformer_imgs/frog/frogo.png")
    {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Failed to load texture: {}", e);
            return;
        }
    };
    texture.set_scale_mode(ScaleMode::Nearest); // to avoid blurring

    let mut sprite = Sprite::new(&texture, 2, vec![6, 4], 52, 50);

    let sprite_pos = Vec2d { x: 250, y: 150 };
    let sprite_scale = Vec2d { x: 4, y: 4 };
    let mut current_animation = Animation::Idle;

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
                // Change animation when pressing space
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    if current_animation == Animation::Idle {
                        current_animation = Animation::Walk;
                    } else {
                        current_animation = Animation::Idle;
                    }
                    let animation_u32 = current_animation.clone() as u32;
                    sprite.start_animation(&animation_u32);
                }
                _ => {}
            }
        }

        // Update
        sprite.update();

        // Render
        frug_instance.clear(background_color);
        frug_instance.draw_sprite(&sprite, &sprite_pos, &sprite_scale);
        frug_instance.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
