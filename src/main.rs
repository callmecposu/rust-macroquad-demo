use macroquad::prelude::*;

use rand::gen_range;

use std::rc::Rc;
use std::cell::RefCell;

const tile_size_px: i32 = 24;
const map_width: usize = 25;
const map_height: usize = 25;
const wall_prob: f32 = 0.2;
const character_speed_per_sec: f32 = (2 * tile_size_px) as f32;
const hitbox_x_margin: f32 = 0.1;
const hitbox_y_margin: f32 = 0.1;

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
                {
                    let root_ref = level.root_elem.borrow();
                    let map_container_ref = root_ref.children[0].borrow();
                    let character_rc = map_container_ref.children
                        .last()
                        .unwrap();
                    handle_walk_input(character_rc, &level);
                }
                {
                    // update the levelcontainer's x & y to be centered

                    let root_ref = level.root_elem.borrow();
                    let mut map_container_ref =
                        root_ref.children[0].borrow_mut();
                    map_container_ref.x =
                        screen_width() / 2.0 - map_container_ref.width / 2.0;
                    map_container_ref.y =
                        screen_height() / 2.0 - map_container_ref.height / 2.0;
                }
                level.root_elem.borrow().draw();
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

fn handle_walk_input(character_rc: &Rc<RefCell<ScreenObject>>, level: &Level) {
    let mut character_ref = character_rc.borrow_mut();
    let character_speed = character_speed_per_sec / get_fps() as f32;
    // respond to user input
    if is_key_down(KeyCode::W) {
        if
            !level.collides_with_a_wall(
                character_ref.x,
                character_ref.y - character_speed
            )
        {
            character_ref.y -= character_speed;
        }
    }
    if is_key_down(KeyCode::A) {
        if
            !level.collides_with_a_wall(
                character_ref.x - character_speed,
                character_ref.y
            )
        {
            character_ref.x -= character_speed;
        }
    }
    if is_key_down(KeyCode::S) {
        if
            !level.collides_with_a_wall(
                character_ref.x,
                character_ref.y + character_speed
            )
        {
            character_ref.y += character_speed;
        }
    }
    if is_key_down(KeyCode::D) {
        if
            !level.collides_with_a_wall(
                character_ref.x + character_speed,
                character_ref.y
            )
        {
            character_ref.x += character_speed;
        }
    }
}
