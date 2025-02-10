use frug::{Color, Event, Keycode};
use std::ops::AddAssign;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Clone, Copy)]
struct Vector2<T> {
    x: T,
    y: T,
}

// Implement AddAssign trait for Vector2
impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct CollisionRectangle {
    pos: Vector2<i32>,
    width: i32,
    height: i32,
    vel: Vector2<i32>,
}

impl CollisionRectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> CollisionRectangle {
        CollisionRectangle {
            pos: Vector2 { x, y },
            width,
            height,
            vel: Vector2 { x: 0, y: 0 },
        }
    }

    fn update_pos(&mut self) {
        self.pos += self.vel;
    }

    /// This function will only work for the ball!
    fn check_collision(&mut self, obj: &CollisionRectangle) {
        // Horizontal collision
        if self.pos.x < obj.pos.x + obj.width && self.pos.x + self.width > obj.pos.x {
            if self.pos.y < obj.pos.y + obj.height && self.pos.y + self.height > obj.pos.y {
                // Horizontal collision
                if self.pos.x + self.width / 2 < obj.pos.x + obj.width / 2 {
                    // Left collision
                    self.vel.x = -self.vel.x.abs();
                } else {
                    // Right collision
                    self.vel.x = self.vel.x.abs();
                }

                // Vertical collision
                if self.pos.y + self.height / 2 < obj.pos.y + obj.height / 2 {
                    // Top collision
                    self.vel.y = -self.vel.y.abs();
                } else {
                    // Bottom collision
                    self.vel.y = self.vel.y.abs();
                }
            }
        }

        // No collision was detected
    }

    fn check_collision_screen(&mut self) {
        if self.pos.y >= HEIGHT as i32 {
            if self.vel.y > 0 {
                self.vel.y *= -1;
            }
        } else if self.pos.y < 0 {
            if self.vel.y < 0 {
                self.vel.y *= -1;
            }
        }
    }

    fn is_game_over(&mut self) -> bool {
        if self.pos.x >= WIDTH as i32 || self.pos.x + self.width < 0 {
            return true;
        }
        return false;
    }
}

fn main() {
    // Initialize context and create window
    let context = frug::init().unwrap();
    let mut canvas = frug::create_window(&context, WIDTH, HEIGHT);

    // our objects
    let mut ball: CollisionRectangle =
        CollisionRectangle::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32, 20, 20);
    let mut opponent = CollisionRectangle::new((WIDTH - 30) as i32, (HEIGHT / 2) as i32, 20, 100);
    let mut player = CollisionRectangle::new(10, (HEIGHT / 2) as i32, 20, 100);

    let background_color = Color::RGB(50, 50, 50);

    canvas.set_draw_color(background_color);
    canvas.clear();
    canvas.present();

    let mut event_pump = context.event_pump().unwrap();
    let paddle_speed = 6;
    let c_white = Color::RGB(255, 255, 255);
    'running: loop {
        // Pre-draw
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
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    // start moving the ball if it's not moving
                    if ball.vel.x == 0 {
                        let dir = 1;
                        ball.vel.x = 6 * dir;
                        ball.vel.y = 6 * -dir;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player.vel.y = -paddle_speed;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    player.vel.y = paddle_speed;
                }
                _ => {
                    player.vel.y = 0;
                }
            }
        }

        //              ** Game loop here **
        // move opponent
        let opponent_center_y = opponent.pos.y + (opponent.height / 2);
        let ball_center_y = ball.pos.y + (ball.height / 2);
        if opponent_center_y > ball_center_y {
            opponent.vel.y = -paddle_speed;
        } else if opponent_center_y < ball_center_y {
            opponent.vel.y = paddle_speed;
        } else {
            opponent.vel.y = 0;
        }

        // bounce ball in case of collision
        ball.check_collision(&opponent);
        ball.check_collision(&player);
        ball.check_collision_screen();

        // check if it's game over
        if ball.is_game_over() {
            // restart ball
            ball.pos.x = (WIDTH / 2) as i32;
            ball.pos.y = (HEIGHT / 2) as i32;
            ball.vel.x = 0;
            ball.vel.y = 0;
        }

        // updates
        ball.update_pos();
        opponent.update_pos();
        player.update_pos();

        // reset player velocity

        // render objects
        frug::draw_rectangle(
            &mut canvas,
            c_white,
            ball.pos.x as i32,
            ball.pos.y as i32,
            ball.width as u32,
            ball.height as u32,
        );
        frug::draw_rectangle(
            &mut canvas,
            c_white,
            player.pos.x as i32,
            player.pos.y as i32,
            player.width as u32,
            player.height as u32,
        );
        frug::draw_rectangle(
            &mut canvas,
            c_white,
            opponent.pos.x as i32,
            opponent.pos.y as i32,
            opponent.width as u32,
            opponent.height as u32,
        );
        //              ** End of game loop **

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
