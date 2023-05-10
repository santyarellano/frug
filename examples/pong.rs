use cgmath::Vector2;
use rand::Rng;

extern crate frug;

struct CollisionRectangle {
    pos: Vector2<f32>,
    width: f32,
    height: f32,
    vel: Vector2<f32>,
}

impl CollisionRectangle {
    fn new(x: f32, y: f32, width: f32, height: f32) -> CollisionRectangle {
        CollisionRectangle {
            pos: Vector2 { x, y },
            width,
            height,
            vel: Vector2 { x: 0.0, y: 0.0 },
        }
    }

    fn update_pos(&mut self) {
        self.pos += self.vel;
    }

    /// This function will only work for the ball!
    fn check_collision(&mut self, obj: &CollisionRectangle) {
        let mut rng = rand::thread_rng();

        let tolerance_w = 0.15 * self.width; // percentage of shape
        let tolerance_h = 0.15 * self.height; // percentage of shape

        // Horizontal collision
        if self.pos.y - tolerance_h > obj.pos.y - obj.height {
            if self.pos.y - tolerance_h < obj.pos.y {
                // give priority for horizontal collision
                if self.pos.x < obj.pos.x + obj.width {
                    if self.pos.x > obj.pos.x {
                        // Left collision is happening
                        self.vel.x *= -1.0;
                        self.vel.y = rng.gen_range(-0.01..0.01);
                        return;
                    }
                }

                if self.pos.x + self.width > obj.pos.x {
                    if self.pos.x + self.width < obj.pos.x + obj.width {
                        // Right collision is happening
                        self.vel.x *= -1.0;
                        self.vel.y = rng.gen_range(-0.01..0.01);
                        return;
                    }
                }
            }
        }

        // Vertical collision
        if self.pos.x + tolerance_w < obj.pos.x - obj.width {
            if self.pos.x + tolerance_w > obj.pos.x {
                // give priority for vertical collision
                if self.pos.y > obj.pos.y - obj.height {
                    if self.pos.y < obj.pos.y {
                        // Up collision is happening
                        self.vel.y *= -1.0;
                        return;
                    }
                }

                if self.pos.y - self.height < obj.pos.y {
                    if self.pos.y - self.height > obj.pos.y - obj.height {
                        // Down collision is happening
                        self.vel.y *= -1.0;
                        return;
                    }
                }
            }
        }

        // No collision was detected
    }

    fn check_collision_screen(&mut self) {
        if self.pos.y >= 1.0 {
            if self.vel.y > 0.0 {
                self.vel.y *= -1.0;
            }
        } else if self.pos.y - self.height < -1.0 {
            if self.vel.y < 0.0 {
                self.vel.y *= -1.0;
            }
        }
    }

    fn is_game_over(&mut self) -> bool {
        if self.pos.x - self.width * 2.0 >= 1.0 || self.pos.x + self.width * 2.0 < -1.0 {
            return true;
        }
        return false;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let (mut frug_instance, event_loop) = frug::new("Pong!");

    frug_instance.set_window_size(800.0, 800.0);

    // our objects
    let mut ball: CollisionRectangle = CollisionRectangle::new(-0.05, -0.05, 0.1, 0.1);
    let mut opponent = CollisionRectangle::new(0.8, 0.2, 0.1, 0.5);
    let mut player = CollisionRectangle::new(-0.9, 0.2, 0.1, 0.5);

    let update_function = move |instance: &mut frug::FrugInstance, input: &frug::InputHelper| {
        let paddle_speed = 0.006;

        // start moving the ball if it's not moving
        if input.key_pressed(frug::VirtualKeyCode::Return) {
            if ball.vel.x == 0.0 {
                let dir = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
                ball.vel.x = 0.01 * dir;
            }
        }

        // move opponent
        let opponent_y = opponent.pos.y - (opponent.height / 2.0);
        let ball_y = ball.pos.y - (ball.height / 2.0);
        if opponent_y > ball_y {
            opponent.vel.y = -paddle_speed;
        } else if opponent_y < ball_y {
            opponent.vel.y = paddle_speed;
        }

        // move player
        player.vel.y = 0.0;
        if input.key_held(frug::VirtualKeyCode::Up) {
            player.vel.y += paddle_speed;
        }
        if input.key_held(frug::VirtualKeyCode::Down) {
            player.vel.y -= paddle_speed;
        }

        // bounce ball in case of collision
        ball.check_collision(&opponent);
        ball.check_collision(&player);
        ball.check_collision_screen();

        // check if it's game over
        if ball.is_game_over() {
            // restart ball
            ball.pos.x = -0.05;
            ball.pos.y = -0.05;
            ball.vel.x = 0.0;
            ball.vel.y = 0.0;
        }

        // updates
        ball.update_pos();
        opponent.update_pos();
        player.update_pos();

        // Rendering
        instance.clear();

        // render objects
        instance.add_colored_rect(
            ball.pos.x,
            ball.pos.y,
            ball.width,
            ball.height,
            [1.0, 1.0, 1.0],
        );
        instance.add_colored_rect(
            opponent.pos.x,
            opponent.pos.y,
            opponent.width,
            opponent.height,
            [1.0, 1.0, 1.0],
        );
        instance.add_colored_rect(
            player.pos.x,
            player.pos.y,
            player.width,
            player.height,
            [1.0, 1.0, 1.0],
        );

        instance.update_buffers();
    };

    frug_instance.run(event_loop, update_function);
}
