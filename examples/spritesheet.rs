use frug::{Event, Instance, Keycode, LoadTexture, Rect, Sprite};

enum Animation {
    Idle,
    Walk,
}

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", 800, 600);
    let texture_creator = frug_instance.new_texture_creator();

    // load the spritesheet
    let texture = match texture_creator.load_texture("examples/platformer_imgs/frog/frogo.png") {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Failed to load texture: {}", e);
            return;
        }
    };

    let mut sprite = Sprite::new(texture, 2, vec![6, 4], 52, 50);
    sprite.start_animation(Animation::Idle as u32);

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

        // Update
        sprite.update();

        // Render
        frug_instance.clear();
        frug_instance.draw_sprite(&sprite, 200, 200);
        frug_instance.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
