use cgmath::Vector2;
use frug::FrugInstance;

extern crate frug;

// ======= CONSTANTS & ENUMS ======
const GRAVITY: f32 = 0.001;

enum Collision {
    Up,
    Down,
    Left,
    Right,
    None,
}
// ======= CONSTANTS & ENUMS ======

/// This function helps us draw the same texture for our background on repeat.
fn draw_repeat_background(instance: &mut frug::FrugInstance, tex_idx: usize, rows: u16, cols: u16) {
    let tile_w: f32 = 2.0 / cols as f32;
    let tile_h: f32 = 2.0 / rows as f32;
    for i in 0..rows {
        for j in 0..cols {
            instance.add_tex_rect(
                tile_w * j as f32 - 1.0,
                tile_h * -(i as f32) + 1.0,
                tile_w,
                tile_h,
                tex_idx,
            );
        }
    }
}

// ======= OUR ECS STRUCTS AND IMPLEMENTATIONS ======
#[derive(Clone)]
struct Entity {
    tex_idx: Option<usize>,
    pos: Option<Vector2<f32>>,
    vel: Option<Vector2<f32>>,
    size: Option<Vector2<f32>>,
    collisions: bool,
    gravity: bool,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            tex_idx: None,
            pos: None,
            vel: None,
            size: None,
            collisions: false,
            gravity: false,
        }
    }
}

impl Entity {
    fn update(&mut self, all_entities: &[Entity], current_idx: usize) {
        // gravity
        if self.gravity {
            self.vel.as_mut().unwrap().y -= GRAVITY;
        }

        // collisions
        if self.collisions {
            for i in 0..all_entities.len() {
                if i != current_idx {
                    // only collide if other object has collisions as well
                    if all_entities[i].collisions {
                        match self.check_collision(&all_entities[i]) {
                            Collision::Up => {
                                match self.vel.as_mut() {
                                    Some(v) => {
                                        if v.y > 0.0 {
                                            //self.vel.as_mut().unwrap() = 0.0;
                                            v.y = 0.0;
                                        }
                                    }
                                    None => {}
                                }
                            }
                            Collision::Down => match self.vel.as_mut() {
                                Some(v) => {
                                    if v.y < 0.0 {
                                        v.y = 0.0;
                                    }
                                }
                                None => {}
                            },
                            Collision::Left => {
                                println!("colliding left");
                            }
                            Collision::Right => {
                                println!("colliding right");
                            }
                            Collision::None => {}
                        }
                    }
                }
            }
        }

        // movement
        match self.vel {
            Some(v) => {
                let pos = self.pos.unwrap();
                self.pos = Some(pos + v);
            }
            None => {}
        }
    }

    fn render(&self, frug_instance: &mut FrugInstance) {
        match self.tex_idx {
            Some(idx) => frug_instance.add_tex_rect(
                self.pos.unwrap().x,
                self.pos.unwrap().y,
                self.size.unwrap().x,
                self.size.unwrap().y,
                idx,
            ),
            None => {}
        }
    }

    // checks collision with another entity
    fn check_collision(&self, other: &Entity) -> Collision {
        let self_right = self.pos.unwrap().x + self.size.unwrap().x;
        let self_bottom = self.pos.unwrap().y + self.size.unwrap().y;
        let other_right = other.pos.unwrap().x + other.size.unwrap().x;
        let other_bottom = other.pos.unwrap().y + other.size.unwrap().y;

        if self.pos.unwrap().y <= other_bottom
            && self_bottom >= other.pos.unwrap().y
            && self.pos.unwrap().x <= other_right
            && self_right >= other.pos.unwrap().x
        {
            let horiz_dist = ((self.pos.unwrap().x + self.size.unwrap().x / 2.0)
                - (other.pos.unwrap().x + other.size.unwrap().x / 2.0))
                .abs();
            let vert_dist = ((self.pos.unwrap().y + self.size.unwrap().y / 2.0)
                - (other.pos.unwrap().y + other.size.unwrap().y / 2.0))
                .abs();

            if vert_dist > horiz_dist {
                if self.pos.unwrap().y + self.size.unwrap().y / 2.0
                    < other.pos.unwrap().y + other.size.unwrap().y / 2.0
                {
                    Collision::Up
                } else {
                    Collision::Down
                }
            } else {
                if self.pos.unwrap().x + self.size.unwrap().x / 2.0
                    < other.pos.unwrap().x + other.size.unwrap().x / 2.0
                {
                    Collision::Left
                } else {
                    Collision::Right
                }
            }
        } else {
            Collision::None
        }
    }
}
// ======= OUR ECS STRUCTS AND IMPLEMENTATIONS ======

fn main() {
    let (mut frug_instance, event_loop) = frug::new("My Window");

    // ======= WINDOW SETUP ======
    frug_instance.set_window_size(800.0, 800.0);
    // ======= WINDOW SETUP ======

    // ======= LOAD ASSETS ======
    // background
    let img_bytes = include_bytes!("platformer_imgs/Purple.png");
    let background_tex_idx = frug_instance.load_texture(img_bytes);

    // land
    let img_bytes = include_bytes!("platformer_imgs/land.png");
    let land_tex_idx = frug_instance.load_texture(img_bytes);

    // frog
    let frog_tex_idxs = vec![
        frug_instance.load_texture(include_bytes!("platformer_imgs/frog/0.png")),
        frug_instance.load_texture(include_bytes!("platformer_imgs/frog/1.png")),
        frug_instance.load_texture(include_bytes!("platformer_imgs/frog/2.png")),
        frug_instance.load_texture(include_bytes!("platformer_imgs/frog/3.png")),
    ];
    // ======= LOAD ASSETS ======

    // ======= GAME VARIABLES ======
    let world_matrix = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 1, 0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    // ======= GAME VARIABLES ======

    // ======= ENTITIES ======
    let mut entities: Vec<Entity> = Vec::new();

    // load entities according to world matrix
    for i in 0..world_matrix.len() {
        for j in 0..world_matrix[0].len() {
            match world_matrix[i][j] {
                1 => {
                    // lands
                    let land_size = 2.0 / world_matrix.len() as f32;
                    let land_pos_x = j as f32 * land_size - 1.0;
                    let land_pos_y = i as f32 * -land_size + 1.0;
                    let new_land = Entity {
                        tex_idx: Some(land_tex_idx),
                        pos: Some(Vector2 {
                            x: land_pos_x,
                            y: land_pos_y,
                        }),
                        size: Some(Vector2 {
                            x: land_size,
                            y: land_size,
                        }),
                        collisions: true,
                        ..Default::default()
                    };

                    entities.push(new_land);
                }
                2 => {
                    // player
                    let size = 2.0 / world_matrix.len() as f32;
                    let pos_x = j as f32 * size - 1.0;
                    let pos_y = i as f32 * -size + 1.0;
                    let new_player = Entity {
                        tex_idx: Some(frog_tex_idxs[0]),
                        pos: Some(Vector2 { x: pos_x, y: pos_y }),
                        size: Some(Vector2 { x: size, y: size }),
                        vel: Some(Vector2 { x: 0.0, y: 0.0 }),
                        collisions: true,
                        gravity: true,
                        ..Default::default()
                    };

                    entities.push(new_player);
                }
                _ => {}
            }
        }
    }
    // ======= ENTITIES ======

    // ***** THE UPDATE FUNCTION *****
    let update_function = move |instance: &mut frug::FrugInstance, _input: &frug::InputHelper| {
        // update
        for i in 0..entities.len() {
            let mut current_entity = entities[i].clone();

            current_entity.update(&entities, i);
            entities[i] = current_entity;
        }

        // ======= RENDER ======
        instance.clear();
        // background
        draw_repeat_background(instance, background_tex_idx, 6, 6);

        // entities
        for entity in &entities {
            entity.render(instance);
        }

        // present
        instance.update_buffers();
        // ======= RENDER ======
    };
    // ***** THE UPDATE FUNCTION *****

    frug_instance.run(event_loop, update_function);
}
