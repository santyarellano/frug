use frug::Graphics;

fn main() {
    let mut graphics = Graphics::new("Spritesheet Example", 800, 600);

    /*let spritesheet = match graphics.load_image("path/to/spritesheet.png") {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Failed to load spritesheet: {}", e);
            return;
        }
    };*/

    graphics.clear();

    // Draw a sprite from the spritesheet
    graphics.draw_sprite(&spritesheet, 0, 0, 32, 32, 100, 100, 64, 64);

    graphics.present();
}
