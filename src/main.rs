use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod ray_casting_engine;

fn main() {
    const MAP_X: usize = 24;
    const MAP_Y: usize = 24;

    // Rectangle to clear old pixels
    let screen: sdl2::rect::Rect = sdl2::rect::Rect::new(0, 0,
                                   ray_casting_engine::SCREEN_WIDTH,
                                   ray_casting_engine::SCREEN_HEIGHT);

    // Player character information
    let mut character = ray_casting_engine::Location {
        pos: ray_casting_engine::Vector {
            x: 10_f32,
            y: 12_f32
        },
        dir: ray_casting_engine::Vector {
            x: 0_f32,
            y: -1_f32
        },
        plane: ray_casting_engine::Vector {
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
        "Wolfenstein 3D", ray_casting_engine::SCREEN_WIDTH,
        ray_casting_engine::SCREEN_HEIGHT)
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
                    ray_casting_engine::move_forwards(&game_map, &mut character, &move_speed);
                    },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    ray_casting_engine::move_backwards(&game_map, &mut character, &move_speed);
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    ray_casting_engine::move_left(&game_map, &mut character, &move_speed);
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    ray_casting_engine::move_right(&game_map, &mut character, &move_speed);
                },                
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    ray_casting_engine::rotate_left(&mut character, rotate_speed);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    ray_casting_engine::rotate_right(&mut character, rotate_speed);
                },
                _ => {}
            }
        }

        let mut x: u32 = 0;

        while x < ray_casting_engine::SCREEN_WIDTH {
            let camera_x: f32 = 2 as f32 * x as f32 / ray_casting_engine::SCREEN_WIDTH as f32 - 1 as f32;
            let ray_dir = ray_casting_engine::Vector {
                x: character.dir.x+character.plane.x*camera_x,
                y: character.dir.y+character.plane.y*camera_x
            };

            let mut map = (character.pos.x as i32, character.pos.y as i32);

            let delta_dist = ray_casting_engine::Vector {
                x: if ray_dir.x == 0_f32 {f32::INFINITY} else {f32::abs(1 as f32 / ray_dir.x)},
                y: if ray_dir.y == 0_f32 {f32::INFINITY} else {f32::abs(1 as f32 / ray_dir.y)}
            };

            let mut hit: u8 = 0;
            let mut side: i8 = 0;

            let mut step_dir = (0 as i8, 0 as i8);

            let mut side_dist = ray_casting_engine::Vector {
                x: 0 as f32,
                y: 0 as f32
            };

            ray_casting_engine::ray_to_step(&ray_dir, &character, &map, &delta_dist,
                        &mut step_dir, &mut side_dist);

            // DDA Algorithm
            ray_casting_engine::dda_alg(&game_map, &mut hit, &mut side_dist, &mut map, &mut side,
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

            let perp_wall_dist: f32 = ray_casting_engine::perp_wall_dist(&side, &side_dist, &delta_dist);

            let line_height: u32 = ray_casting_engine::vert_pixels(&perp_wall_dist);

            let mut draw_line = ((ray_casting_engine::SCREEN_HEIGHT - line_height) / 2,
                        (ray_casting_engine::SCREEN_HEIGHT + line_height) / 2);

            if draw_line.1 >= ray_casting_engine::SCREEN_HEIGHT {draw_line.1 = ray_casting_engine::SCREEN_HEIGHT - 1}

            let point1 = sdl2::rect::Point::new(x as i32, draw_line.0 as i32);
            let point2 = sdl2::rect::Point::new(x as i32, draw_line.1 as i32);

            let color: ray_casting_engine::Color = ray_casting_engine::pixel_color(game_map, &map, &side);

            canvas.set_draw_color(color);
            let _error_check = canvas.draw_line(point1, point2);

            x += 1;
        }

        canvas.present();
        canvas.set_draw_color(ray_casting_engine::Color::BLACK);
        let _error_check = canvas.fill_rect(screen);

    }
}