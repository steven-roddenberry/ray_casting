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
    const SCREEN_WIDTH: u32 = 1280;
    const SCREEN_HEIGHT: u32 = 720;
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
            x: -0.66_f32,
            y: 0_f32
        }
    };

    // Camera and Character movement rates
    let move_speed = 0.1;
    let rotate_speed = 0.06;

    // Current wall layout
    let game_map: [[i32; MAP_X]; MAP_Y] = 
    [
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
        [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
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
                    if game_map[(character.pos.x + character.dir.x * move_speed) as usize]
                               [(character.pos.y) as usize] == 0 
                        {character.pos.x += character.dir.x * move_speed};
                    if game_map[(character.pos.x) as usize]
                               [(character.pos.y + character.dir.y * move_speed) as usize] == 0
                        {character.pos.y += character.dir.y * move_speed};
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    if game_map[(character.pos.x - character.dir.x * move_speed) as usize]
                               [(character.pos.y) as usize] == 0 
                        {character.pos.x -= character.dir.x * move_speed};
                    if game_map[(character.pos.x) as usize]
                               [(character.pos.y - character.dir.y * move_speed) as usize] == 0
                        {character.pos.y -= character.dir.y * move_speed};
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
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
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
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
                },
                _ => {}
            }
        }

        // 
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

            // DDA Algorithm
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

            let perp_wall_dist: f32;

            if side == 0 {
                perp_wall_dist = side_dist.x - delta_dist.x;
            }
            else {
                perp_wall_dist = side_dist.y - delta_dist.y;
            }

            let line_height: u32;

            if perp_wall_dist < 1 as f32 {
                line_height = SCREEN_HEIGHT - 1;
            }
            else {
                line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist as f32) as u32;
            }

            let mut draw_line = ((SCREEN_HEIGHT - line_height) / 2,
                        (SCREEN_HEIGHT + line_height) / 2);

            if draw_line.1 >= SCREEN_HEIGHT {draw_line.1 = SCREEN_HEIGHT - 1}

            let point1 = sdl2::rect::Point::new(x as i32, draw_line.0 as i32);
            let point2 = sdl2::rect::Point::new(x as i32, draw_line.1 as i32);

            let mut color: Color;

            match game_map[map.0 as usize][map.1 as usize] {
                1 => color = Color::RED,
                2 => color = Color::GREEN,
                3 => color = Color::BLUE,
                4 => color = Color::WHITE,
                _ => color = Color::YELLOW
            }

            if side == 1 {
                color.r = color.r / 2;
                color.g = color.g / 2;
                color.b = color.b / 2;
            }

            canvas.set_draw_color(color);
            canvas.draw_line(point1, point2);

            x += 1;
        }

        canvas.present();
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(screen);

    }
}
