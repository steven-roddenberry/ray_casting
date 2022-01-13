use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

struct Location {
    pos: Vector,
    dir: Vector,
    plane: Vector
}

struct Vector {
    x: f32,
    y: f32
}

fn main() {
    const SCREEN_WIDTH: u32 = 1920;
    const SCREEN_HEIGHT: u32 = 1080;
    const MAP_X: usize = 24;
    const MAP_Y: usize = 24;

    // Rectangle to clear old pixels
    let screen: sdl2::rect::Rect = sdl2::rect::Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    // Player character information
    let mut character = Location {
        pos: Vector {
            x: 10_f32,
            y: 12_f32
        },
        dir: Vector {
            x: 0_f32,
            y: -1_f32
        },
        plane: Vector {
            x: -1_f32,
            y: 0_f32
        }
    };

    // Camera and Character movement rates
    let move_speed = 0.15;
    let rotate_speed = 0.0628318531;        // ~ 2 * pi / 100

    // Sample wall layout
    let game_map: [[i32; MAP_X]; MAP_Y] = 
    [
        [4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,7,7,7,7,7,7,7,7],
        [4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
        [4,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
        [4,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
        [4,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
        [4,0,4,0,0,0,0,5,5,5,5,5,5,5,5,5,7,7,0,7,7,7,7,7],
        [4,0,5,0,0,0,0,5,0,5,0,5,0,5,0,5,7,0,0,0,7,7,7,1],
        [4,0,6,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
        [4,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,1],
        [4,0,8,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
        [4,0,0,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,7,7,7,1],
        [4,0,0,0,0,0,0,5,5,5,5,0,5,5,5,5,7,7,7,7,7,7,7,1],
        [6,6,6,6,6,6,6,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
        [8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4],
        [6,6,6,6,6,6,0,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
        [4,4,4,4,4,4,0,4,4,4,6,0,6,2,2,2,2,2,2,2,3,3,3,3],
        [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
        [4,0,0,0,0,0,0,0,0,0,0,0,6,2,0,0,5,0,0,2,0,0,0,2],
        [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
        [4,0,6,0,6,0,0,0,0,4,6,0,0,0,0,0,5,0,0,0,0,0,0,2],
        [4,0,0,5,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
        [4,0,6,0,6,0,0,0,0,4,6,0,6,2,0,0,5,0,0,2,0,0,0,2],
        [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
        [4,4,4,4,4,4,4,4,4,4,1,1,1,2,2,2,2,2,2,3,3,3,3,3]
    ];

    // Open and initialize program window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
        "Wolfenstein 3D", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Begin game loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                    move_forwards(&game_map, &mut character, &move_speed);
                    },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    move_backwards(&game_map, &mut character, &move_speed);
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    move_left(&game_map, &mut character, &move_speed);
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    move_right(&game_map, &mut character, &move_speed);
                },                
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    rotate_left(&mut character, rotate_speed);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rotate_right(&mut character, rotate_speed);
                },
                _ => {}
            }
        }

        let mut x: u32 = 0;

        while x < SCREEN_WIDTH {
            let camera_x: f32 = 2 as f32 * x as f32 / SCREEN_WIDTH as f32 - 1 as f32;
            let ray_dir = Vector {
                x: character.dir.x+character.plane.x*camera_x,
                y: character.dir.y+character.plane.y*camera_x
            };

            let mut map = (character.pos.x as i32, character.pos.y as i32);

            let delta_dist = Vector {
                x: if ray_dir.x == 0_f32 {f32::INFINITY} else {f32::abs(1 as f32 / ray_dir.x)},
                y: if ray_dir.y == 0_f32 {f32::INFINITY} else {f32::abs(1 as f32 / ray_dir.y)}
            };

            let mut hit: u8 = 0;
            let mut side: i8 = 0;

            let mut step_dir = (0 as i8, 0 as i8);

            let mut side_dist = Vector {
                x: 0 as f32,
                y: 0 as f32
            };

            ray_to_step(&ray_dir, &character, &map, &delta_dist,
                        &mut step_dir, &mut side_dist);

            // DDA Algorithm
            dda_alg(&game_map, &mut hit, &mut side_dist, &mut map, &mut side,
                    &delta_dist, &step_dir);
            while hit == 0 {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map.0 += step_dir.0 as i32;
                    side = 0;
                }
                else {
                    side_dist.y += delta_dist.y;
                    map.1 += step_dir.1 as i32;
                    side = 1;
                }
                if game_map[map.0 as usize][map.1 as usize] != 0 { hit = 1 }
            }

            let perp_wall_dist: f32 = perp_wall_dist(&side, &side_dist, &delta_dist);

            let line_height: u32 = vert_pixels(&perp_wall_dist, &SCREEN_HEIGHT);

            let mut draw_line = ((SCREEN_HEIGHT - line_height) / 2,
                        (SCREEN_HEIGHT + line_height) / 2);

            if draw_line.1 >= SCREEN_HEIGHT {draw_line.1 = SCREEN_HEIGHT - 1}

            let point1 = sdl2::rect::Point::new(x as i32, draw_line.0 as i32);
            let point2 = sdl2::rect::Point::new(x as i32, draw_line.1 as i32);

            let color: Color = pixel_color(game_map, &map, &side);

            canvas.set_draw_color(color);
            let _error_check = canvas.draw_line(point1, point2);

            x += 1;
        }

        canvas.present();
        canvas.set_draw_color(Color::BLACK);
        let _error_check = canvas.fill_rect(screen);

    }
}

// Movement Functions

fn move_forwards<const MAP_X: usize, const MAP_Y: usize>
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

fn move_backwards<const MAP_X: usize, const MAP_Y: usize>
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

fn move_left<const MAP_X: usize, const MAP_Y: usize>
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

fn move_right<const MAP_X: usize, const MAP_Y: usize>
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

fn rotate_left(mut character: &mut Location, rotate_speed: f32) {

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

fn rotate_right(mut character: &mut Location, rotate_speed: f32) {

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

fn ray_to_step(ray_dir: &Vector,
               character: &Location,
               map: &(i32, i32),
               delta_dist: &Vector,
               mut step_dir: &mut (i8, i8),
               mut side_dist: &mut Vector) {

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

fn dda_alg<const MAP_X: usize, const MAP_Y: usize>
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

fn perp_wall_dist(side: &i8, side_dist: &Vector, delta_dist: &Vector) -> f32 {

    if *side == 0 {
        side_dist.x - delta_dist.x
    }
    else {
        side_dist.y - delta_dist.y
    }

}

// Display Function s

fn vert_pixels(perp_wall_dist: &f32, screen_height: &u32) -> u32 {

    if *perp_wall_dist < 1 as f32 {
        *screen_height - 1
    }
    else {
        (*screen_height as f32 / *perp_wall_dist) as u32
    }

}

fn pixel_color<const MAP_X: usize, const MAP_Y: usize>
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