//! The `frug::run` game loop: fixed 60 Hz updates, a low-resolution world scaled up to the
//! window, a camera, held/pressed keys, alpha/tint/flip drawing and a window-space HUD.
//!
//! Arrows move the frog, Space flashes it, Esc quits.
use frug::{
    run, Color, Config, Context, DrawParams, Game, Keycode, TextureId, Vec2d,
};

struct Demo {
    frog: TextureId,
    pos: Vec2d<f32>,
    facing_left: bool,
    flash: u32,
}

impl Game for Demo {
    fn update(&mut self, ctx: &mut Context) {
        if ctx.input.pressed(Keycode::Escape) {
            ctx.quit();
        }
        let dx = ctx.input.axis(Keycode::Left, Keycode::Right) as f32;
        let dy = ctx.input.axis(Keycode::Up, Keycode::Down) as f32;
        self.pos.x += dx * 1.5;
        self.pos.y += dy * 1.5;
        if dx != 0.0 {
            self.facing_left = dx < 0.0;
        }
        if ctx.input.pressed(Keycode::Space) {
            self.flash = 6;
        }
        self.flash = self.flash.saturating_sub(1);
        // Camera follows the frog.
        let (w, h) = ctx.instance.logical_size();
        ctx.instance.camera = Vec2d {
            x: self.pos.x - w as f32 / 2.0,
            y: self.pos.y - h as f32 / 2.0,
        };
        // Smoke-test hook: quit automatically after N updates.
        if let Some(n) = std::env::var("FRUG_QUIT_AFTER")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
        {
            if ctx.tick >= n {
                ctx.quit();
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        // World: a checkerboard so camera movement is visible.
        for gy in -10..10 {
            for gx in -10..10 {
                if (gx + gy) % 2 == 0 {
                    ctx.instance.fill_rect(
                        gx as f32 * 16.0,
                        gy as f32 * 16.0,
                        16.0,
                        16.0,
                        Color::RGB(40, 40, 70),
                    );
                }
            }
        }
        let tex = if self.flash > 0 {
            ctx.assets.flash_texture(self.frog).unwrap()
        } else {
            ctx.assets.texture(self.frog)
        };
        let (fw, fh) = ctx.assets.size(self.frog);
        ctx.instance.draw_texture_ex(
            tex,
            self.pos,
            &DrawParams {
                origin: Vec2d {
                    x: fw as f32 / 2.0,
                    y: fh as f32,
                },
                scale: Vec2d {
                    x: if self.facing_left { -0.25 } else { 0.25 },
                    y: 0.25,
                },
                ..Default::default()
            },
        );

        // HUD in window pixels.
        ctx.instance.begin_gui();
        ctx.instance
            .fill_rect(16.0, 16.0, 200.0, 24.0, Color::RGBA(255, 255, 255, 128));
    }

    fn clear_color(&self) -> Color {
        Color::RGB(20, 20, 35)
    }
}

fn main() {
    run(
        Config::new("Game loop example", 960, 540).logical_size(240, 135),
        |ctx| Demo {
            frog: ctx
                .assets
                .load_texture_with_flash("examples/frog.png")
                .expect("load frog"),
            pos: Vec2d { x: 0.0, y: 0.0 },
            facing_left: false,
            flash: 0,
        },
    );
}
