use frug::{Color, Event, Instance, Keycode, Vec2d};

struct GameObject {
    pos: Vec2d<i32>,
    size: Vec2d<u32>,
    vel: Vec2d<i32>,
    color: Color,
    speed: i32,
}

impl GameObject {
    fn get_center(&self) -> Vec2d<i32> {
        Vec2d {
            x: self.pos.x + (self.size.x / 2) as i32,
            y: self.pos.y + (self.size.y / 2) as i32,
        }
    }

    fn update(&mut self) {
        self.pos += self.vel;
    }

    fn render(&self, instance: &mut Instance) {
        instance.draw_rect(&self.pos, &self.size, self.color);
    }

    fn collide_w_object(&mut self, obj: &GameObject) {
        // Check vertical range
        if self.pos.y < obj.pos.y + obj.size.y as i32 && self.pos.y + self.size.y as i32 > obj.pos.y
        {
            // Check horizontal range
            if self.pos.x < obj.pos.x + obj.size.x as i32
                && self.pos.x + self.size.x as i32 > obj.pos.x
            {
                self.vel.x *= -1;
            }
        }
    }
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            pos: Vec2d { x: 0, y: 0 },
            size: Vec2d { x: 0, y: 0 },
            vel: Vec2d { x: 0, y: 0 },
            color: Color::RGB(255, 255, 255),
            speed: 2,
        }
    }
}

fn main() {
    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;

    let mut frug_instance = Instance::new("Pong", WINDOW_WIDTH, WINDOW_HEIGHT);
    let background_color = Color::RGB(0, 0, 0);

    // Create game items
    let mut player = GameObject {
        pos: Vec2d {
            x: 50,
            y: (WINDOW_HEIGHT / 2 - 80 / 2) as i32,
        },
        size: Vec2d { x: 10, y: 80 },
        ..Default::default()
    };
    let mut enemy = GameObject {
        pos: Vec2d {
            x: (WINDOW_WIDTH - 50 - 10) as i32,
            y: player.pos.y,
        },
        size: player.size.clone(),
        ..Default::default()
    };
    let mut ball = GameObject {
        pos: Vec2d {
            x: (WINDOW_WIDTH / 2 - 5) as i32,
            y: player.pos.y,
        },
        size: Vec2d { x: 10, y: 10 },
        ..Default::default()
    };

    // Start ball
    ball.vel.x = ball.speed;
    ball.vel.y = -ball.speed;

    // Player controls
    let mut player_up = 0;
    let mut player_down = 0;

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
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player_up = 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player_up = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    player_down = 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    player_down = 0;
                }
                _ => {}
            }
        }

        // Control player
        player.vel.y = (player_down - player_up) * player.speed;

        // Control enemy
        let ball_center_y = ball.get_center().y;
        let enemy_center_y = enemy.get_center().y;
        if ball_center_y < enemy_center_y {
            enemy.vel.y = -enemy.speed;
        } else if ball_center_y > enemy_center_y {
            enemy.vel.y = enemy.speed;
        }

        // ** Ball collisions **
        // Vertical borders
        if ball.pos.y <= 0 || ball.pos.y + ball.size.y as i32 >= WINDOW_HEIGHT as i32 {
            ball.vel.y *= -1;
        }
        // With pallets
        ball.collide_w_object(&player);
        ball.collide_w_object(&enemy);
        // ** Ball collisions **

        // Update
        player.update();
        enemy.update();
        ball.update();

        // Render
        frug_instance.clear(background_color);
        player.render(&mut frug_instance);
        enemy.render(&mut frug_instance);
        ball.render(&mut frug_instance);
        frug_instance.present();

        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
