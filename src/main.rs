use macroquad::prelude::*;
use macroquad::experimental::animation::*;

use rand::gen_range;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::{ RefCell, RefMut };

const tile_size_px: i32 = 64;
const map_width: usize = 25;
const map_height: usize = 25;
const wall_prob: f32 = 0.2;
const character_speed_per_sec: f32 = (2 * tile_size_px) as f32;
const hitbox_x_margin: f32 = (tile_size_px / 8) as f32;
const hitbox_y_margin: f32 = (tile_size_px / 8) as f32;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Demo"),
        window_width: 1200,
        window_height: 700,
        ..Default::default()
    }
}

struct Level {
    map: Vec<Vec<u8>>,
    root_elem: Rc<RefCell<ScreenObject>>,
}

impl Level {
    fn new() -> Level {
        let map = generate_map();
        let root = Rc::new(
            RefCell::new(ScreenObject {
                x: 0.0,
                y: 0.0,
                width: screen_width(),
                height: screen_height(),
                color: LIGHTGRAY,
                parent: None,
                children: Vec::new(),
            })
        );
        return Level { map, root_elem: root };
    }

    fn prepare(&self) -> &Self {
        // create map container
        let map_container = Rc::new(
            RefCell::new(ScreenObject {
                x: screen_width() / 2.0 -
                (((map_width as i32) * tile_size_px) as f32) / 2.0,
                y: screen_height() / 2.0 -
                (((map_height as i32) * tile_size_px) as f32) / 2.0,
                width: ((map_width as i32) * tile_size_px) as f32,
                height: ((map_height as i32) * tile_size_px) as f32,
                color: WHITE,
                parent: Some(Rc::clone(&self.root_elem)),
                children: Vec::new(),
            })
        );
        self.root_elem.borrow_mut().children.push(Rc::clone(&map_container));
        // add wall tiles according to map as children
        for map_y in 0..map_height {
            for map_x in 0..map_width {
                if self.map[map_y][map_x] == 1 {
                    let wall_tile = Rc::new(
                        RefCell::new(ScreenObject {
                            x: ((map_x as i32) * tile_size_px) as f32,
                            y: ((map_y as i32) * tile_size_px) as f32,
                            width: tile_size_px as f32,
                            height: tile_size_px as f32,
                            color: GRAY,
                            parent: Some(Rc::clone(&map_container)),
                            children: Vec::new(),
                        })
                    );
                    map_container
                        .borrow_mut()
                        .children.push(Rc::clone(&wall_tile));
                }
            }
        }
        // create a character object
        let (mut char_map_x, mut char_map_y) = (0, 0);
        while self.map[char_map_y][char_map_x] != 0 {
            char_map_x = gen_range(0, map_width);
            char_map_y = gen_range(0, map_width);
        }
        let character = Rc::new(
            RefCell::new(ScreenObject {
                x: ((char_map_x as i32) * tile_size_px) as f32,
                y: ((char_map_y as i32) * tile_size_px) as f32,
                width: tile_size_px as f32,
                height: tile_size_px as f32,
                color: YELLOW,
                parent: Some(Rc::clone(&map_container)),
                children: Vec::new(),
            })
        );
        map_container.borrow_mut().children.push(Rc::clone(&character));
        // center camera on the player
        map_container.borrow_mut().x =
            screen_width() / 2.0 - character.borrow().x;
        map_container.borrow_mut().y =
            screen_height() / 2.0 - character.borrow().y;
        // map_x + char_x = screen_w / 2
        return &self;
    }

    fn collides_with_a_wall(&self, x: f32, y: f32) -> bool {
        let corner_coordinates = vec![
            get_map_position(x + hitbox_x_margin, y + hitbox_y_margin),
            get_map_position(
                x + (tile_size_px as f32) - hitbox_x_margin,
                y + hitbox_y_margin
            ),
            get_map_position(
                x + hitbox_x_margin,
                y + (tile_size_px as f32) - hitbox_y_margin
            ),
            get_map_position(
                x + (tile_size_px as f32) - hitbox_x_margin,
                y + (tile_size_px as f32) - hitbox_y_margin
            )
        ];
        println!("corner_coordinates: {:?}", corner_coordinates);
        return corner_coordinates
            .iter()
            .any(|coord| self.map[coord.1][coord.0] == 1);
    }
}

enum AppState {
    Menu,
    Game,
    AnimationTest,
}

struct ScreenObject {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    parent: Option<Rc<RefCell<ScreenObject>>>,
    children: Vec<Rc<RefCell<ScreenObject>>>,
}

impl ScreenObject {
    fn get_absolute_position(&self) -> (f32, f32) {
        if let Some(parent) = &self.parent {
            return (
                self.x + parent.borrow().get_absolute_position().0,
                self.y + parent.borrow().get_absolute_position().1,
            );
        } else {
            return (self.x, self.y);
        }
    }

    fn draw(&self) {
        // assuming all the shapes for now are rectangles

        // draw the object
        draw_rectangle(
            self.get_absolute_position().0,
            self.get_absolute_position().1,
            self.width,
            self.height,
            self.color
        );
        // draw the children
        self.children.iter().for_each(|child| {
            child.borrow().draw();
        });
    }
}

#[macroquad::main(conf)]
async fn main() {
    let level = Level::new();
    level.prepare();

    let mut app_state = AppState::Menu;

    // define animations
    let mut sprite = AnimatedSprite::new(
        64,
        64,
        &[
            Animation {
                name: "idle".to_string(),
                row: 24,
                frames: 2,
                fps: 3,
            },
            Animation {
                name: "walkNorth".to_string(),
                row: 8,
                frames: 9,
                fps: 8,
            },
            Animation {
                name: "walkWest".to_string(),
                row: 9,
                frames: 9,
                fps: 8,
            },
            Animation {
                name: "walkSouth".to_string(),
                row: 10,
                frames: 9,
                fps: 8,
            },
            Animation {
                name: "walkEast".to_string(),
                row: 11,
                frames: 9,
                fps: 8,
            },
        ],
        true
    );

    let spritesheet = load_texture(
        "images/spritesheet_demo.png"
    ).await.unwrap();

    let mut anim_buffer = Vec::<usize>::new();

    loop {
        match app_state {
            AppState::Menu => {
                if is_key_down(KeyCode::Enter) {
                    app_state = AppState::Game;
                }
                clear_background(WHITE);
                let welcome_text = "Hello World!";
                let welcome_font_size = 30;
                let welcome_text_size = measure_text(
                    &welcome_text,
                    None,
                    welcome_font_size,
                    1.0
                );
                draw_text(
                    welcome_text,
                    screen_width() / 2.0 - welcome_text_size.width / 2.0,
                    screen_height() / 2.0 - welcome_text_size.height / 2.0,
                    welcome_font_size as f32,
                    BLACK
                );
                // animate the text appearing and disappearing every second
                let action_text = "Press <Enter> to get started...";
                let action_text_font_size = 16;
                let action_text_size = measure_text(
                    &action_text,
                    None,
                    action_text_font_size,
                    1.0
                );
                let time = get_time();
                if time % 2.0 < 1.0 {
                    draw_text(
                        &action_text,
                        screen_width() / 2.0 - action_text_size.width / 2.0,
                        screen_height() / 2.0 -
                            welcome_text_size.height / 2.0 +
                            50.0,
                        action_text_font_size as f32,
                        BLACK
                    );
                }
            }
            AppState::Game => {
                clear_background(LIGHTGRAY);
                handle_walk_input(&level);
                level.root_elem.borrow().draw();
                {
                    let root_ref = level.root_elem.borrow();
                    let map_container_ref = root_ref.children[0].borrow();
                    let character_ref = map_container_ref.children
                        .iter()
                        .last()
                        .unwrap()
                        .borrow();
                    animate_walking(
                        &mut sprite,
                        &spritesheet,
                        &mut anim_buffer,
                        character_ref.get_absolute_position().0,
                        character_ref.get_absolute_position().1
                    );
                }
            }
            AppState::AnimationTest => {
                animate_walking(
                    &mut sprite,
                    &spritesheet,
                    &mut anim_buffer,
                    10.0,
                    10.0
                );
            }
        }

        next_frame().await;
    }
}

fn generate_map() -> Vec<Vec<u8>> {
    let mut map = vec![vec![0 as u8; map_width]; map_height];

    for y in 0..map_height {
        for x in 0..map_width {
            if x == 0 || x == map_width - 1 || y == 0 || y == map_height - 1 {
                map[y][x] = 1;
            } else if gen_range(0.0, 1.0) < wall_prob {
                map[y][x] = 1;
            }
        }
    }

    return map;
}

fn get_map_position(x: f32, y: f32) -> (usize, usize) {
    return (
        (x / (tile_size_px as f32)).floor() as usize,
        (y / (tile_size_px as f32)).floor() as usize,
    );
}

fn handle_walk_input(level: &Level) {
    let (mut new_x, mut new_y): (f32, f32);
    let (mut orig_x, mut orig_y): (f32, f32);
    {
        let root_ref = level.root_elem.borrow();
        let map_container_ref = root_ref.children[0].borrow();
        let character_ref = map_container_ref.children.last().unwrap().borrow();
        let character_speed = character_speed_per_sec / (get_fps() as f32);
        (new_x, new_y) = (character_ref.x, character_ref.y);
        (orig_x, orig_y) = (character_ref.x, character_ref.y);
        // respond to user input
        if is_key_down(KeyCode::W) {
            if !level.collides_with_a_wall(new_x, new_y - character_speed) {
                new_y -= character_speed;
            }
        }
        if is_key_down(KeyCode::A) {
            if !level.collides_with_a_wall(new_x - character_speed, new_y) {
                new_x -= character_speed;
            }
        }
        if is_key_down(KeyCode::S) {
            if !level.collides_with_a_wall(new_x, new_y + character_speed) {
                new_y += character_speed;
            }
        }
        if is_key_down(KeyCode::D) {
            if !level.collides_with_a_wall(new_x + character_speed, new_y) {
                new_x += character_speed;
            }
        }
    }
    move_character(level, new_x - orig_x, new_y - orig_y);
}

fn is_within_bounds(x: f32, y: f32, bounds: ((f32, f32), (f32, f32))) -> bool {
    x >= bounds.0.0 &&
        x + (tile_size_px as f32) <= bounds.1.0 &&
        y >= bounds.0.1 &&
        y + (tile_size_px as f32) <= bounds.1.1
}

fn move_character(level: &Level, x_diff: f32, y_diff: f32) {
    let camera_bounds = (
        (screen_width() / 4.0, screen_height() / 4.0),
        ((screen_width() * 3.0) / 4.0, (screen_height() * 3.0) / 4.0),
    );

    let root_ref = level.root_elem.borrow();
    let (mut char_screen_x, mut char_screen_y): (f32, f32);
    {
        let map_container_ref = root_ref.children[0].borrow();
        let character_ref = map_container_ref.children.last().unwrap().borrow();
        (char_screen_x, char_screen_y) = character_ref.get_absolute_position();
    }

    char_screen_x += x_diff;
    char_screen_y += y_diff;

    {
        let map_container_ref = root_ref.children[0].borrow();
        let mut character_ref = map_container_ref.children
            .last()
            .unwrap()
            .borrow_mut();
        character_ref.x += x_diff;
        character_ref.y += y_diff;
    }

    if !is_within_bounds(char_screen_x, char_screen_y, camera_bounds) {
        let mut map_container_ref = root_ref.children[0].borrow_mut();
        map_container_ref.x -= x_diff;
        map_container_ref.y -= y_diff;
    }
}

fn animate_walking(
    sprite: &mut AnimatedSprite,
    spritesheet: &Texture2D,
    anim_buffer: &mut Vec<usize>,
    x: f32,
    y: f32
) {
    // clear_background(WHITE);
    draw_texture_ex(&spritesheet, x, y, WHITE, DrawTextureParams {
        source: Some(sprite.frame().source_rect),
        dest_size: Some(sprite.frame().dest_size),
        ..Default::default()
    });
    // change animation depending on user input
    if is_key_down(KeyCode::W) {
        if !anim_buffer.contains(&1) {
            anim_buffer.push(1);
        }
    }
    if is_key_down(KeyCode::A) {
        if !anim_buffer.contains(&2) {
            anim_buffer.push(2);
        }
    }
    if is_key_down(KeyCode::S) {
        if !anim_buffer.contains(&3) {
            anim_buffer.push(3);
        }
    }
    if is_key_down(KeyCode::D) {
        if !anim_buffer.contains(&4) {
            anim_buffer.push(4);
        }
    }
    // remove animations from anim_buffer on key release
    if is_key_released(KeyCode::W) {
        if let Some(rm_index) = anim_buffer.iter().position(|x| *x == 1) {
            anim_buffer.remove(rm_index);
        }
    }
    if is_key_released(KeyCode::A) {
        if let Some(rm_index) = anim_buffer.iter().position(|x| *x == 2) {
            anim_buffer.remove(rm_index);
        }
    }
    if is_key_released(KeyCode::S) {
        if let Some(rm_index) = anim_buffer.iter().position(|x| *x == 3) {
            anim_buffer.remove(rm_index);
        }
    }
    if is_key_released(KeyCode::D) {
        if let Some(rm_index) = anim_buffer.iter().position(|x| *x == 4) {
            anim_buffer.remove(rm_index);
        }
    }
    // if anim buffer containes something, switch to the last animation
    if anim_buffer.len() > 0 {
        let last_added_anim = *anim_buffer.iter().last().unwrap();
        if sprite.current_animation() != last_added_anim {
            sprite.set_animation(last_added_anim);
            sprite.set_frame(1);
        }
    } else {
        // reset to idle
        if sprite.current_animation() != 0 {
            sprite.set_animation(0);
            sprite.set_frame(0);
        }
    }
    // if walking, skip the first frame
    if
        (1..=4).contains(&sprite.current_animation()) &&
        sprite.frame().source_rect.x == 0.0
    {
        sprite.set_frame(1);
    }
    sprite.update();
}
