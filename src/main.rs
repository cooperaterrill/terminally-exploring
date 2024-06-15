//extern crate ncurses;
//TODO: massive lag spike when entering new chunk (probably reduce chunk size to smooth)

use ncurses::*;
use std::collections::{HashMap, HashSet};
const RENDER_SIZE: usize = 41;

const CHUNK_SIZE: i32 = 300;
const LAKE_CHANCE: f64 = 0.005;
const MOUNTAIN_RANGE_CHANCE: f64 = 0.02;

const GRASS_PAIR: i16 = 1;
const MOUNTAIN_PAIR: i16 = 2;
const WATER_PAIR: i16 = 3;
const COLOR_MAX_WHITE: i16 = 33;
const COLOR_DARK_BLUE: i16 = 34;
const COLOR_DARK_GREEN: i16 = 35;

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

fn main() {
    initscr();
    init_colors();
    game_loop();
}

fn init_colors() {
    start_color();
    init_color(COLOR_MAX_WHITE, 1000, 1000, 1000);
    init_color(COLOR_DARK_BLUE, 200, 200, 800);
    init_color(COLOR_DARK_GREEN, 200, 800, 200);
    init_pair(GRASS_PAIR, COLOR_YELLOW, COLOR_DARK_GREEN);
    init_pair(MOUNTAIN_PAIR, COLOR_MAX_WHITE, COLOR_BLACK);
    init_pair(WATER_PAIR, COLOR_BLUE, COLOR_DARK_BLUE);
}

fn game_loop() {
    let mut player_coords: Pos = Pos { x: 1000, y: 1000 };
    let mut map: HashMap<Pos, Terrain> = HashMap::new();
    let mut generated_chunks: HashSet<Pos> = HashSet::new();
    generate_chunk(calc_chunk(&player_coords), &mut map, &mut generated_chunks);

    //generate_terrain(Terrain::Mountain, &Pos { x: 1001, y: 1001 }, &mut map);
    //let top_left = Pos { x: 995, y: 995 };
    //let bot_right = Pos { x: 1005, y: 999 };
    //generate_terrain_rectangle(Terrain::Mountain, top_left, bot_right, &mut map);
    //generate_terrain_circle_chance(Terrain::Water, Pos { x: 1010, y: 1010 }, 5, &mut map);
    //if map.get(&Pos { x: 1001, y: 1001 }).unwrap() == &Terrain::Mountain {
    //    let _ = addstr("Terrain generatement successful");
    //} else {
    //    let _ = addstr("Couldn't add terrain");
    //}
    refresh();
    //TODO: game plan: randomly generate terrain in chunks
    loop {
        let input: i32 = getch();
        clear();
        //let _ = addstr(format!("Got input {}", input).as_str());
        match parse_input(input) {
            Some(act) => {
                take_action(act, &mut player_coords, &mut map, &mut generated_chunks);
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

fn take_action(
    action: Action,
    player_coords_ref: &mut Pos,
    map_ref: &mut HashMap<Pos, Terrain>,
    generated_chunks_ref: &mut HashSet<Pos>,
) {
    match action {
        Action::Up => {
            try_move(
                Pos {
                    x: player_coords_ref.x,
                    y: player_coords_ref.y + 1,
                },
                player_coords_ref,
                map_ref,
                generated_chunks_ref,
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
                generated_chunks_ref,
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
                generated_chunks_ref,
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
                generated_chunks_ref,
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

fn try_move(
    goal_coords: Pos,
    player_coords_ref: &mut Pos,
    map_ref: &mut HashMap<Pos, Terrain>,
    generated_chunks_ref: &mut HashSet<Pos>,
) {
    if *map_ref.get(&goal_coords).unwrap() == Terrain::Grass {
        player_coords_ref.x = goal_coords.x;
        player_coords_ref.y = goal_coords.y;

        //go through in 3x3 grid to render all neighbouring chunks
        let this_chunk = calc_chunk(&goal_coords);
        for x_chunk_offset in -1..2 {
            for y_chunk_offset in -1..2 {
                let curr_chunk = Pos {
                    x: this_chunk.x + x_chunk_offset * CHUNK_SIZE,
                    y: this_chunk.y + y_chunk_offset * CHUNK_SIZE,
                };

                if !generated_chunks_ref.contains(&curr_chunk) {
                    generate_chunk(curr_chunk, map_ref, generated_chunks_ref);
                }
            }
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
                color_set(GRASS_PAIR);
                let _ = addstr(" @ ");
                //let _ = addstr("RENDERING PLAYE");
                continue;
            }

            let loc: Pos = Pos { x: myx, y: myy };
            render_terrain_spot(map_ref.get(&loc).unwrap());
        }
        let _ = addstr("\n");
    }
}

fn render_terrain_spot(terrain: &Terrain) {
    match terrain {
        Terrain::Grass => {
            color_set(GRASS_PAIR);
            let _ = addstr("   ");
            //let _ = addstr("RENDERING GRASS");
        }
        Terrain::Mountain => {
            color_set(MOUNTAIN_PAIR);
            let _ = addstr(" ^ ");
            //let _ = addstr("RENDERING MOUNT");
        }
        Terrain::Water => {
            color_set(WATER_PAIR);
            let _ = addstr(" ~ ");
        }
    }
}

fn generate_chunk(
    top_left_corner: Pos,
    map_ref: &mut HashMap<Pos, Terrain>,
    generated_chunks_ref: &mut HashSet<Pos>,
) {
    for currx in top_left_corner.x..top_left_corner.x + CHUNK_SIZE {
        for curry in top_left_corner.y..top_left_corner.y + CHUNK_SIZE {
            let mut rand: f64 = rand::random::<f64>();

            if rand < LAKE_CHANCE {
                let radius: i32 = rand::random::<i32>() % 6 + 1;
                generate_terrain_circle_chance(
                    Terrain::Water,
                    Pos { x: currx, y: curry },
                    radius,
                    map_ref,
                );
                continue;
            }

            rand = rand::random::<f64>();
            if rand < MOUNTAIN_RANGE_CHANCE {
                //let the current pos be the top left
                let width: i32 = rand::random::<i32>() % 40 - 20;
                let length: i32 = rand::random::<i32>() % 40 - 20;
                generate_terrain_rectangle(
                    Terrain::Mountain,
                    Pos { x: currx, y: curry },
                    Pos {
                        x: currx + length,
                        y: curry + width,
                    },
                    map_ref,
                );
            }
        }
    }

    for x_offset in 0..CHUNK_SIZE {
        for y_offset in 0..CHUNK_SIZE {
            let loc = Pos {
                x: top_left_corner.x + x_offset,
                y: top_left_corner.y + y_offset,
            };
            if !map_ref.contains_key(&loc) {
                map_ref.insert(loc, Terrain::Grass);
            }
        }
    }

    generated_chunks_ref.insert(top_left_corner);
}

fn generate_terrain(terrain: Terrain, loc: &Pos, map_ref: &mut HashMap<Pos, Terrain>) {
    let _ = map_ref.insert(*loc, terrain);
}

fn generate_terrain_chance(
    prob: f64,
    terrain: Terrain,
    loc: &Pos,
    map_ref: &mut HashMap<Pos, Terrain>,
) {
    //let _ = addstr(&format!("Got prob of {}", prob));
    if rand::random::<f64>() < prob {
        generate_terrain(terrain, loc, map_ref);
    }
}

//TODO: make sure it looks good
//TODO: precomputed r^2?
//TODO: STOP CREATING NEW POS
fn generate_terrain_circle_chance(
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
            generate_terrain_chance(
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

//TODO: optimize to not create new pos every time
fn generate_terrain_rectangle(
    terrain: Terrain,
    top_left_corner: Pos,
    bot_right_corner: Pos,
    map_ref: &mut HashMap<Pos, Terrain>,
) {
    for currx in top_left_corner.x..bot_right_corner.x + 1 {
        for curry in top_left_corner.y..bot_right_corner.y + 1 {
            //let _ = addstr(format!("Rendered at {}, {}\n", currx, curry).as_str());
            generate_terrain(terrain.clone(), &Pos { x: currx, y: curry }, map_ref);
        }
    }
}

//TODO: skip unneeded checks he3e, stop creating new poses
fn generate_terrain_circle(
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
                generate_terrain(
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

fn calc_chunk(loc: &Pos) -> Pos {
    return Pos {
        x: CHUNK_SIZE * (loc.x / CHUNK_SIZE),
        y: CHUNK_SIZE * (loc.y / CHUNK_SIZE),
    };
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

fn quit() {
    endwin();
    std::process::exit(0);
}
