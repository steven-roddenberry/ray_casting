pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;    

pub use sdl2::pixels::Color;

pub struct Location {
    pub pos: Vector,
    pub dir: Vector,
    pub plane: Vector
}

pub struct Vector {
    pub x: f32,
    pub y: f32
}    

pub fn move_forwards<const MAP_X: usize, const MAP_Y: usize>
                (game_map: &[[i32; MAP_X]; MAP_Y],
                    character: &mut Location,
                    move_speed: & f32) {

    if game_map[(character.pos.x + character.dir.x * move_speed) as usize]
                [(character.pos.y) as usize] == 0 
        {character.pos.x += character.dir.x * move_speed};

    if game_map[(character.pos.x) as usize]
                [(character.pos.y + character.dir.y * move_speed) as usize] == 0
        {character.pos.y += character.dir.y * move_speed};

}

pub fn move_backwards<const MAP_X: usize, const MAP_Y: usize>
                    (game_map: &[[i32; MAP_X]; MAP_Y],
                    character: &mut Location,
                    move_speed: & f32) {

    if game_map[(character.pos.x - character.dir.x * move_speed) as usize]
                [(character.pos.y) as usize] == 0 
        {character.pos.x -= character.dir.x * move_speed};

    if game_map[(character.pos.x) as usize]
                [(character.pos.y - character.dir.y * move_speed) as usize] == 0
        {character.pos.y -= character.dir.y * move_speed};

}

pub fn move_left<const MAP_X: usize, const MAP_Y: usize>
            (game_map: &[[i32; MAP_X]; MAP_Y],
                character: &mut Location,
                move_speed: & f32) {

    if game_map[(character.pos.x - character.plane.x * move_speed) as usize]
                [(character.pos.y) as usize] == 0 
        {character.pos.x -= character.plane.x * move_speed};

    if game_map[(character.pos.x) as usize]
                [(character.pos.y - character.plane.y * move_speed) as usize] == 0
        {character.pos.y -= character.plane.y * move_speed};

}

pub fn move_right<const MAP_X: usize, const MAP_Y: usize>
                (game_map: &[[i32; MAP_X]; MAP_Y],
                character: &mut Location,
                move_speed: & f32) {

    if game_map[(character.pos.x + character.plane.x * move_speed) as usize]
                [(character.pos.y) as usize] == 0 
        {character.pos.x += character.plane.x * move_speed};

    if game_map[(character.pos.x) as usize]
                [(character.pos.y + character.plane.y * move_speed) as usize] == 0
        {character.pos.y += character.plane.y * move_speed};

}


// Camera Functions

pub fn rotate_left(character: &mut Location, rotate_speed: f32) {

    let old_dir = character.dir.x;
    character.dir.x = old_dir * f32::cos(rotate_speed)
                    - character.dir.y * f32::sin(rotate_speed);
    character.dir.y = old_dir * f32::sin(rotate_speed)
                    + character.dir.y * f32::cos(rotate_speed);

    let old_plane = character.plane.x;
    character.plane.x = old_plane * f32::cos(rotate_speed)
                        - character.plane.y * f32::sin(rotate_speed);
    character.plane.y = old_plane * f32::sin(rotate_speed)
                        + character.plane.y * f32::cos(rotate_speed);

}

pub fn rotate_right(character: &mut Location, rotate_speed: f32) {

    let old_dir = character.dir.x;
    character.dir.x = old_dir * f32::cos(-rotate_speed)
                    - character.dir.y * f32::sin(-rotate_speed);
    character.dir.y = old_dir * f32::sin(-rotate_speed)
                    + character.dir.y * f32::cos(-rotate_speed);

    let old_plane = character.plane.x;
    character.plane.x = old_plane * f32::cos(-rotate_speed)
                        - character.plane.y * f32::sin(-rotate_speed);
    character.plane.y = old_plane * f32::sin(-rotate_speed)
                        + character.plane.y * f32::cos(-rotate_speed);

}


// Render Data Functions

pub fn ray_to_step(ray_dir: &Vector,
                character: &Location,
                map: &(i32, i32),
                delta_dist: &Vector,
                step_dir: &mut (i8, i8),
                side_dist: &mut Vector) {

    if ray_dir.x < 0 as f32 {
        step_dir.0 = -1;
        side_dist.x = (character.pos.x - map.0 as f32) * delta_dist.x;
    }
    else {
        step_dir.0 = 1;
        side_dist.x = (map.0 as f32 + 1 as f32 - character.pos.x) * delta_dist.x;
    }
    if ray_dir.y < 0 as f32 {
        step_dir.1 = -1;
        side_dist.y = (character.pos.y - map.1 as f32) * delta_dist.y;
    }
    else {
        step_dir.1 = 1;
        side_dist.y = (map.1 as f32 + 1 as f32 - character.pos.y) * delta_dist.y;
    }

}

pub fn dda_alg<const MAP_X: usize, const MAP_Y: usize>
            (game_map: &[[i32; MAP_X]; MAP_Y],
            hit: &mut u8,
            side_dist: &mut Vector,
            map: &mut (i32, i32),
            side: &mut i8,
            delta_dist: &Vector,
            step_dir: &(i8, i8)) {

    while *hit == 0 as u8 {
        if side_dist.x < side_dist.y {
            side_dist.x += delta_dist.x;
            map.0 += step_dir.0 as i32;
            *side = 0;
        }
        else {
            side_dist.y += delta_dist.y;
            map.1 += step_dir.1 as i32;
            *side = 1;
        }
        if game_map[map.0 as usize][map.1 as usize] != 0 { *hit = 1}
    }
}

pub fn perp_wall_dist(side: &i8, side_dist: &Vector, delta_dist: &Vector) -> f32 {

    if *side == 0 {
        side_dist.x - delta_dist.x
    }
    else {
        side_dist.y - delta_dist.y
    }
}


// Display Functions

pub fn vert_pixels(perp_wall_dist: &f32) -> u32 {

    if *perp_wall_dist < 1 as f32 {
        SCREEN_HEIGHT - 1
    }
    else {
        (SCREEN_HEIGHT as f32 / *perp_wall_dist) as u32
    }
}

pub fn pixel_color<const MAP_X: usize, const MAP_Y: usize>
                (game_map: [[i32; MAP_X]; MAP_Y],
                map: &(i32, i32),
                side: &i8) -> Color {

    let mut color: Color;

    match game_map[map.0 as usize][map.1 as usize] {
        1 => color = Color::RED,
        2 => color = Color::GREEN,
        3 => color = Color::BLUE,
        4 => color = Color::WHITE,
        5 => color = Color::CYAN,
        6 => color = Color::MAGENTA,
        7 => color = Color::GRAY,
        _ => color = Color::YELLOW
    }

    if *side == 1 {
        color.r = color.r / 2;
        color.g = color.g / 2;
        color.b = color.b / 2;
    }
    color
}