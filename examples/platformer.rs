use cgmath::Vector2;
use frug::FrugInstance;

extern crate frug;

// ======= CONSTANTS & ENUMS ======
const GRAVITY: f32 = 0.001;
const PLAYER_SPEED: f32 = 0.01;
const PLAYER_JUMP: f32 = 0.02;

enum Collision {
    Top,
    Bottom,
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
                false,
                false,
            );
        }
    }
}

// ======= OUR ECS STRUCTS AND IMPLEMENTATIONS ======
#[derive(Clone)]
struct Sprite {
    tex_idxs: Vec<usize>,
    anim_speed: u8,
    frame_timer: u8,
    current_idx: usize,
}

#[derive(Clone)]
struct Entity {
    _name: Option<String>,
    tex_idx: Option<usize>,
    sprite: Option<Sprite>,
    pos: Option<Vector2<f32>>,
    vel: Option<Vector2<f32>>,
    size: Option<Vector2<f32>>,
    collisions: bool,
    gravity: bool,
    controlling: bool,
    flip_img_x: bool,
    flip_img_y: bool,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            _name: None,
            tex_idx: None,
            sprite: None,
            pos: None,
            vel: None,
            size: None,
            collisions: false,
            gravity: false,
            controlling: false,
            flip_img_x: false,
            flip_img_y: false,
        }
    }
}

impl Entity {
    fn process_input(&mut self, input: &frug::InputHelper) {
        if self.controlling {
            match self.vel.as_mut() {
                Some(vel) => {
                    vel.x = 0.0;
                    if input.key_held(frug::VirtualKeyCode::Left) {
                        vel.x -= PLAYER_SPEED;
                    }

                    if input.key_held(frug::VirtualKeyCode::Right) {
                        vel.x += PLAYER_SPEED;
                    }

                    if input.key_pressed(frug::VirtualKeyCode::Space) {
                        if vel.y == 0.0 {
                            vel.y += PLAYER_JUMP;
                        }
                    }
                }
                None => {}
            }
        }
    }

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
                            Collision::Top => match self.vel.as_mut() {
                                Some(v) => {
                                    if v.y > 0.0 {
                                        v.y = 0.0;
                                    }
                                }
                                None => {}
                            },
                            Collision::Bottom => match self.vel.as_mut() {
                                Some(v) => {
                                    //println!("{}", v.y);
                                    if v.y < 0.0 {
                                        v.y = 0.0;
                                    }
                                }
                                None => {}
                            },
                            Collision::Left => match self.vel.as_mut() {
                                Some(v) => {
                                    if v.x < 0.0 {
                                        v.x = 0.0;
                                    }
                                }
                                None => {}
                            },
                            Collision::Right => match self.vel.as_mut() {
                                Some(v) => {
                                    if v.x > 0.0 {
                                        v.x = 0.0;
                                    }
                                }
                                None => {}
                            },
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

        // animate
        self.animate();
    }

    fn render(&self, frug_instance: &mut FrugInstance) {
        match self.tex_idx {
            Some(idx) => frug_instance.add_tex_rect(
                self.pos.unwrap().x,
                self.pos.unwrap().y,
                self.size.unwrap().x,
                self.size.unwrap().y,
                idx,
                self.flip_img_x,
                self.flip_img_y,
            ),
            None => {}
        }
    }

    // choose the correct texture index accordingly
    fn animate(&mut self) {
        // animate only if object has sprites
        match self.sprite.as_mut() {
            Some(sprite) => {
                match &self._name {
                    // Define how the animations work for each name
                    Some(name) => match name.as_str() {
                        "Player" => {
                            match &self.vel {
                                Some(vel) => {
                                    // flip img if necessary
                                    if vel.x > 0.0 {
                                        self.flip_img_x = false;
                                    } else if vel.x < 0.0 {
                                        self.flip_img_x = true;
                                    }

                                    // jump/fall (has priority)
                                    if vel.y > 0.0 {
                                        sprite.current_idx = 5;
                                    } else if vel.y < 0.0 {
                                        sprite.current_idx = 4;
                                    }
                                    // walk
                                    else if vel.x != 0.0 {
                                        // update timer
                                        if sprite.frame_timer == 0 {
                                            // timer ended
                                            sprite.frame_timer = sprite.anim_speed;
                                            sprite.current_idx += 1;
                                            if sprite.current_idx > 3 {
                                                // animation must go back to beggining
                                                sprite.current_idx = 0;
                                            }
                                        }
                                        sprite.frame_timer -= 1;
                                    }
                                    // idle
                                    else {
                                        sprite.frame_timer = 0;
                                        sprite.current_idx = 0;
                                    }
                                }
                                None => {}
                            }

                            // update texture index
                            self.tex_idx = Some(sprite.tex_idxs[sprite.current_idx]);
                        }
                        _ => {}
                    },
                    None => {}
                }
            }
            None => {}
        }
    }

    // checks collision with another entity
    fn check_collision(&self, other: &Entity) -> Collision {
        let collision_room = self.size.unwrap() / (2.0 * 3.0) / 2.0;

        // self collision borders
        let s_horizontal = (
            self.get_center().x - collision_room.x, // left
            self.get_center().x + collision_room.x, // right
        );
        let s_vertical = (
            self.get_center().y + collision_room.y, // top
            self.get_center().y - collision_room.y, // bottom
        );

        // other collision borders
        let o_horizontal = (
            other.pos.unwrap().x,                         // left
            other.pos.unwrap().x + other.size.unwrap().x, // right
        );
        let o_vertical = (
            other.pos.unwrap().y,                         // top
            other.pos.unwrap().y - other.size.unwrap().y, // bottom
        );

        // check for vertical collisions
        if (s_horizontal.0 > o_horizontal.0 && s_horizontal.0 < o_horizontal.1)
            || (s_horizontal.1 > o_horizontal.0 && s_horizontal.1 < o_horizontal.1)
        {
            // self is within the horizontal range of other

            // check for top collision
            if self.pos.unwrap().y >= o_vertical.1 && self.pos.unwrap().y <= o_vertical.0 {
                return Collision::Top;
            }
            // check for bottom collision
            else if self.pos.unwrap().y - self.size.unwrap().y <= o_vertical.0
                && self.pos.unwrap().y - self.size.unwrap().y >= o_vertical.1
            {
                return Collision::Bottom;
            }
        }

        // check for horizontal collisions
        if (s_vertical.0 < o_vertical.0 && s_vertical.0 > o_vertical.1)
            || (s_vertical.1 < o_vertical.0 && s_vertical.1 > o_vertical.1)
        {
            // self is within the vertical range of other

            // check for left collision
            if self.pos.unwrap().x < o_horizontal.1 && self.pos.unwrap().x > o_horizontal.0 {
                return Collision::Left;
            }

            // check for right collision
            if self.pos.unwrap().x + self.size.unwrap().x > o_horizontal.0
                && self.pos.unwrap().x + self.size.unwrap().x < o_horizontal.1
            {
                return Collision::Right;
            }
        }

        return Collision::None;
    }

    fn get_center(&self) -> Vector2<f32> {
        return Vector2 {
            x: self.pos.unwrap().x + self.size.unwrap().x / 2.0,
            y: self.pos.unwrap().y - self.size.unwrap().y / 2.0,
        };
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
        frug_instance.load_texture(include_bytes!("platformer_imgs/frog/Fall.png")),
        frug_instance.load_texture(include_bytes!("platformer_imgs/frog/Jump.png")),
    ];
    // ======= LOAD ASSETS ======

    // ======= GAME VARIABLES ======
    let world_matrix = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 2, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
                        _name: Some("Player".to_string()),
                        tex_idx: None,
                        sprite: Some(Sprite {
                            tex_idxs: frog_tex_idxs.clone(),
                            anim_speed: 8,
                            frame_timer: 8,
                            current_idx: 0,
                        }),
                        pos: Some(Vector2 { x: pos_x, y: pos_y }),
                        size: Some(Vector2 { x: size, y: size }),
                        vel: Some(Vector2 { x: 0.0, y: 0.0 }),
                        collisions: true,
                        gravity: true,
                        controlling: true,
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
    let update_function = move |instance: &mut frug::FrugInstance, input: &frug::InputHelper| {
        // process input
        for entity in entities.iter_mut() {
            entity.process_input(input)
        }

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
