extern crate ncurses;

use ncurses::*;
use std::collections::HashMap;
//use std::io::{self, stdin, stdout, Read, Write};
const RENDER_SIZE: usize = 41;
const GRASS_COLOR: i16 = 1;
const MOUNTAIN_COLOR: i16 = 2;
const WATER_COLOR: i16 = 3;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

enum Action {
    Left,
    Right,
    Up,
    Down,
    Quit,
    Inventory,
}

#[derive(Eq, PartialEq, Clone)]
enum Terrain {
    Grass,
    Mountain,
    Water,
}
fn init_colors() {
    start_color();
    init_pair(GRASS_COLOR, COLOR_YELLOW, COLOR_GREEN);
    init_pair(MOUNTAIN_COLOR, 33, COLOR_BLACK);
    init_pair(WATER_COLOR, 33, COLOR_BLUE);
}

fn render_terrain_spot(terrain: &Terrain) {
    match terrain {
        Terrain::Grass => {
            color_set(GRASS_COLOR);
            let _ = addstr("   ");
            //let _ = addstr("RENDERING GRASS");
        }
        Terrain::Mountain => {
            color_set(MOUNTAIN_COLOR);
            let _ = addstr(" ^ ");
            //let _ = addstr("RENDERING MOUNT");
        }
        Terrain::Water => {
            color_set(WATER_COLOR);
            let _ = addstr(" ~ ");
        }
    }
}

fn render(player_coords_ref: &Pos, map_ref: &mut HashMap<Pos, Terrain>) {
    //start_color();
    //init_colors();
    for row in 0i32..RENDER_SIZE as i32 {
        let myy = player_coords_ref.y + row - (RENDER_SIZE as i32) / 2;
        for col in 0i32..RENDER_SIZE as i32 {
            let myx = player_coords_ref.x + col - (RENDER_SIZE as i32) / 2;

            if myy == player_coords_ref.y && myx == player_coords_ref.x {
                color_set(GRASS_COLOR);
                let _ = addstr(" @ ");
                //let _ = addstr("RENDERING PLAYE");
                continue;
            }

            let loc: Pos = Pos { x: myx, y: myy };
            render_terrain_spot(map_ref.entry(loc).or_insert(Terrain::Grass));
        }
        let _ = addstr("\n");
    }
}

fn parse_input(input: i32) -> Option<Action> {
    match input {
        68 => Some(Action::Left),
        67 => Some(Action::Right),
        66 => Some(Action::Up),
        65 => Some(Action::Down),
        101 => Some(Action::Inventory),
        113 => Some(Action::Quit),
        _ => None,
    }
}

fn try_move(goal_coords: Pos, player_coords_ref: &mut Pos, map_ref: &mut HashMap<Pos, Terrain>) {
    if *map_ref.entry(goal_coords).or_insert(Terrain::Grass) == Terrain::Grass {
        player_coords_ref.x = goal_coords.x;
        player_coords_ref.y = goal_coords.y;
    }
}
fn take_action(action: Action, player_coords_ref: &mut Pos, map_ref: &mut HashMap<Pos, Terrain>) {
    match action {
        Action::Up => {
            try_move(
                Pos {
                    x: player_coords_ref.x,
                    y: player_coords_ref.y + 1,
                },
                player_coords_ref,
                map_ref,
            );
        }
        Action::Down => {
            try_move(
                Pos {
                    x: player_coords_ref.x,
                    y: player_coords_ref.y - 1,
                },
                player_coords_ref,
                map_ref,
            );
        }
        Action::Left => {
            try_move(
                Pos {
                    x: player_coords_ref.x - 1,
                    y: player_coords_ref.y,
                },
                player_coords_ref,
                map_ref,
            );
        }
        Action::Right => {
            try_move(
                Pos {
                    x: player_coords_ref.x + 1,
                    y: player_coords_ref.y,
                },
                player_coords_ref,
                map_ref,
            );
        }
        Action::Inventory => {
            let _ = addstr("Support for inventory not yet added\n");
        }
        Action::Quit => {
            quit();
        }
    }
}

fn quit() {
    endwin();
    std::process::exit(0);
}

fn change_color(color: i16, red_amt: i16, green_amt: i16, blue_amt: i16) {
    let mut curr_red: i16 = 0;
    let mut curr_green: i16 = 0;
    let mut curr_blue: i16 = 0;
    color_content(color, &mut curr_red, &mut curr_green, &mut curr_blue);
    init_color(
        color,
        curr_red + red_amt,
        curr_green + green_amt,
        curr_blue + blue_amt,
    );
}

fn place_terrain(terrain: Terrain, loc: &Pos, map_ref: &mut HashMap<Pos, Terrain>) {
    let _ = map_ref.insert(*loc, terrain);
}

fn place_terrain_chance(
    prob: f64,
    terrain: Terrain,
    loc: &Pos,
    map_ref: &mut HashMap<Pos, Terrain>,
) {
    //let _ = addstr(&format!("Got prob of {}", prob));
    if rand::random::<f64>() < prob {
        place_terrain(terrain, loc, map_ref);
    }
}

//TODO: make sure it looks good
//TODO: precomputed r^2?
fn place_terrain_circle_chance(
    terrain: Terrain,
    center: Pos,
    radius: i32,
    map_ref: &mut HashMap<Pos, Terrain>,
) {
    for currx in -radius - 3..radius + 3 {
        //TODO: we're going outside the radius b/c random
        for curry in -radius - 3..radius + 3 {
            let inness: f64 = (radius * radius) as f64
                / ((currx as f64 + 0.5).powf(2.0) + (curry as f64 + 0.5).powf(2.0));
            place_terrain_chance(
                (inness / 1.0).powf(8.0), //TODO: fine tune parameters
                //if inness >= 1.0 {
                //    1.0
                //} else {
                //    (inness / 3.0).powf(2.5) //square the
                //                             //amount to cluster successes close to the guaranteed zone
                //},
                terrain.clone(),
                &Pos {
                    x: currx + center.x,
                    y: curry + center.y,
                },
                map_ref,
            );
        }
    }
}

//TODO: skip unneeded checks he3e
fn place_terrain_circle(
    terrain: Terrain,
    center: Pos,
    radius: i32,
    map_ref: &mut HashMap<Pos, Terrain>,
) {
    for currx in -radius..radius {
        for curry in -radius..radius {
            if (currx as f32 + 0.5).powf(2.0) + (curry as f32 + 0.5).powf(2.0)
                <= (radius * radius) as f32
            {
                place_terrain(
                    terrain.clone(),
                    &Pos {
                        x: currx + center.x,
                        y: curry + center.y,
                    },
                    map_ref,
                );
            }
        }
    }
}

//TODO: optimize to not create new pos every time
fn place_terrain_rectangle(
    terrain: Terrain,
    top_left_corner: Pos,
    bot_right_corner: Pos,
    map_ref: &mut HashMap<Pos, Terrain>,
) {
    for currx in top_left_corner.x..bot_right_corner.x + 1 {
        for curry in top_left_corner.y..bot_right_corner.y + 1 {
            let _ = addstr(format!("Rendered at {}, {}\n", currx, curry).as_str());
            place_terrain(terrain.clone(), &Pos { x: currx, y: curry }, map_ref);
        }
    }
}

fn game_loop() {
    let mut player_coords: Pos = Pos { x: 1000, y: 1000 };
    let mut map: HashMap<Pos, Terrain> = HashMap::new();

    //place_terrain(Terrain::Mountain, &Pos { x: 1001, y: 1001 }, &mut map);
    let top_left = Pos { x: 995, y: 995 };
    let bot_right = Pos { x: 1005, y: 999 };
    place_terrain_rectangle(Terrain::Mountain, top_left, bot_right, &mut map);
    place_terrain_circle_chance(Terrain::Water, Pos { x: 1010, y: 1010 }, 10, &mut map);
    //if map.get(&Pos { x: 1001, y: 1001 }).unwrap() == &Terrain::Mountain {
    //    let _ = addstr("Terrain placement successful");
    //} else {
    //    let _ = addstr("Couldn't add terrain");
    //}
    refresh();
    loop {
        let input: i32 = getch();
        clear();
        //let _ = addstr(format!("Got input {}", input).as_str());
        match parse_input(input) {
            Some(act) => {
                take_action(act, &mut player_coords, &mut map);
            }
            None => {
                //let _ = addstr("Couldn't recognize input");
            }
        }
        //take_action(&input, &player_coords, &map);
        render(&player_coords, &mut map);
        refresh();
    }
}

fn main() {
    initscr();
    init_colors();
    game_loop();
}
