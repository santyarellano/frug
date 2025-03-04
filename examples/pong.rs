use frug::{Color, Event, Instance, Keycode, Vec2d};

// CONSTANTS
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const PALLET_SPEED: i32 = 5;

struct Entity {
    position: Vec2d<i32>,
    velocity: Vec2d<i32>,
    size: Vec2d<u32>,
    color: Color,
    is_player: bool,
    is_ball: bool,
    is_enemy: bool,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            position: Vec2d { x: 0, y: 0 },
            velocity: Vec2d { x: 0, y: 0 },
            size: Vec2d { x: 0, y: 0 },
            color: Color::RGB(255, 255, 255),
            is_player: false,
            is_ball: false,
            is_enemy: false,
        }
    }
}

impl Entity {
    fn process_input(&mut self, input: i32) {
        if self.is_player {
            self.velocity.y = input * PALLET_SPEED;
        }
    }

    fn update(&mut self, ball_y_center: &mut i32) {
        // ball behiavior
        if self.is_ball {
            if self.velocity.x == 0 && self.velocity.y == 0 {
                self.velocity.x = PALLET_SPEED;
                self.velocity.y = PALLET_SPEED;
            }

            // Check collision with the top and bottom of the window
            if self.position.y <= 0 || self.position.y + self.size.y as i32 >= WINDOW_HEIGHT as i32
            {
                self.velocity.y = -self.velocity.y;
            }

            // update the world ball position
            let y_center = self.position.y + self.size.y as i32 / 2;
            *ball_y_center = y_center;
        }

        // enemy behiavior
        if self.is_enemy {
            let y_center = self.position.y + self.size.y as i32 / 2;
            if *ball_y_center < y_center {
                self.velocity.y = -PALLET_SPEED;
            } else if *ball_y_center > y_center {
                self.velocity.y = PALLET_SPEED;
            }
        }

        // update position
        self.position += self.velocity;
    }

    fn render(&self, frug_instance: &mut Instance) {
        frug_instance.draw_rect(&self.position, &self.size, self.color);
    }
}

fn main() {
    let mut frug_instance = Instance::new("Spritesheet Example", WINDOW_WIDTH, WINDOW_HEIGHT);
    let background_color = Color::RGB(0, 0, 0);

    // Game entities
    let mut entities: Vec<Entity> = Vec::new();
    let pallets_width = 15;
    let pallets_height = 60;
    let mut ball_y_center = 50;
    let mut in_up = 0;
    let mut in_down = 0;

    // Player
    let player = Entity {
        position: Vec2d {
            x: (WINDOW_WIDTH / 20) as i32,
            y: (WINDOW_HEIGHT / 2 - pallets_height / 2) as i32, // Center the pallet
        },
        size: Vec2d {
            x: pallets_width,
            y: 50,
        },
        is_player: true,
        ..Default::default()
    };
    entities.push(player);

    // Enemy
    let enemy = Entity {
        position: Vec2d {
            x: (WINDOW_WIDTH - WINDOW_WIDTH / 20 - pallets_width) as i32,
            y: (WINDOW_HEIGHT / 2 - pallets_height / 2) as i32,
        },
        size: Vec2d {
            x: pallets_width,
            y: 50,
        },
        is_enemy: true,
        ..Default::default()
    };
    entities.push(enemy);

    // Ball
    let ball = Entity {
        position: Vec2d {
            x: (WINDOW_WIDTH / 2 - pallets_width / 2) as i32,
            y: (WINDOW_HEIGHT / 2 - pallets_width / 2) as i32,
        },
        size: Vec2d {
            x: pallets_width,
            y: pallets_width,
        },
        is_ball: true,
        ..Default::default()
    };
    entities.push(ball);

    // Game loop
    'running: loop {
        // Receive input
        for event in frug_instance.get_events() {
            match event {
                // Quit the application
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // Player movement
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    in_down = 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    in_down = 0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    in_up = 1;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    in_up = 0;
                }
                _ => {}
            }
        }

        // Process input
        for intity in &mut entities {
            intity.process_input(in_down - in_up);
        }

        // Update
        for intity in &mut entities {
            intity.update(&mut ball_y_center);
        }

        // Render
        frug_instance.clear(background_color);
        for intity in &entities {
            intity.render(&mut frug_instance);
        }
        frug_instance.present();

        // Sleep for a while (you can use a timer for a more precise timing)
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
