use gamestate::{MoveDirection, TurnDirection};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::f32;

pub fn main() {
    let mut renderer = renderer::Renderer::new(1250, 500, "Sdl demo window");

    let map = [
        [true, true, true, true, true, true],
        [true, false, false, false, false, true],
        [true, false, false, true, false, true],
        [true, true, false, true, false, true],
        [true, true, false, false, false, true],
        [true, true, true, true, true, true],
    ]
    .iter()
    .map(|row| Vec::from(row))
    .collect::<Vec<Vec<bool>>>();

    let mut gamestate = gamestate::Gamestate::new(map, 150.0, 150.0, 100, 55);

    renderer.set_background_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.set_wall_color(sdl2::pixels::Color::RGB(67, 255, 20));
    renderer.set_floor_color(sdl2::pixels::Color::RGB(255, 0, 255));
    renderer.set_player_color(sdl2::pixels::Color::RGB(0, 0, 255));
    renderer.set_ray_color(sdl2::pixels::Color::RGB(0, 191, 255));

    match renderer.set_scale(1.5) {
        Ok(_) => {}
        Err(s) => {
            println!("Error setting scale: {s}")
        }
    }

    let mut event_pump = renderer.event_pump();

    struct EventWrapper {
        w: bool,
        s: bool,
        a: bool,
        d: bool,

        last_event: std::time::Instant,
    }

    let mut events = EventWrapper {
        w: false,
        s: false,
        a: false,
        d: false,

        last_event: std::time::Instant::now(),
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: code, .. } => match code {
                    Some(Keycode::W) => events.w = true,
                    Some(Keycode::S) => events.s = true,
                    Some(Keycode::A) => events.a = true,
                    Some(Keycode::D) => events.d = true,
                    _ => {}
                },
                Event::KeyUp { keycode: code, .. } => match code {
                    Some(Keycode::W) => events.w = false,
                    Some(Keycode::S) => events.s = false,
                    Some(Keycode::A) => events.a = false,
                    Some(Keycode::D) => events.d = false,
                    _ => {}
                },

                _ => {}
            }
        }

        let current_time = std::time::Instant::now();
        let delta_time = (current_time - events.last_event).as_nanos();

        events.last_event = current_time;
        if events.w {
            gamestate.player_move(MoveDirection::Forward, delta_time);
        }
        if events.s {
            gamestate.player_move(MoveDirection::Backward, delta_time);
        }
        if events.a {
            gamestate.player_rotate(TurnDirection::Left, delta_time);
        }
        if events.d {
            gamestate.player_rotate(TurnDirection::Right, delta_time);
        }

        // The rest of the game loop goes here...

        renderer.clear_canvas();
        let map_drawing_res = renderer.draw_map(
            gamestate.map_walls(),
            gamestate.map_width(),
            gamestate.map_height(),
            35,
        );
        match map_drawing_res {
            Ok(_) => {}
            Err(s) => {
                println!("Unsuccessful drawing: {s}")
            }
        }

        let (x, y) = gamestate.player_position();

        let rays = gamestate.cast_rays();
        let rays_drawing_res = renderer.draw_rays(
            model_to_map_coordinate(x, y, gamestate.block_size()),
            rays.iter()
                .map(|(x_ray, y_ray, _)| {
                    model_to_map_coordinate(*x_ray, *y_ray, gamestate.block_size())
                })
                .collect::<Vec<sdl2::rect::Point>>(),
        );
        match rays_drawing_res {
            Ok(_) => {}
            Err(s) => {
                println!("Unsuccessful drawing: {s}")
            }
        }

        let player_drawing_res = renderer.draw_player(
            model_to_map_coordinate(x, y, gamestate.block_size()),
            gamestate.player_rotation(),
        );
        match player_drawing_res {
            Ok(_) => {}
            Err(s) => {
                println!("Unsuccessful drawing: {s}")
            }
        }
        let screen_drawing_res = renderer.draw_screen(
            sdl2::rect::Point::new(250, 0),
            sdl2::rect::Point::new(800, 300),
        );

        match screen_drawing_res {
            Ok(_) => {}
            Err(s) => {
                println!("Unsuccessful drawing: {s}")
            }
        }

        println!("\n\n");
        let ray_lengths = rays
            .iter()
            .map(|(ray_x, ray_y, hits_wall)| {
                let x_diff = (ray_x - x).powi(2);
                let y_diff = (ray_y - y).powi(2);

                let dist = 1.0 - (x_diff + y_diff).sqrt() / gamestate.view_distance();
                (dist, *hits_wall)
            })
            .collect::<Vec<(f32, bool)>>();
        let walls_drawing_res = renderer.draw_walls(ray_lengths);
        match walls_drawing_res {
            Ok(_) => {}
            Err(s) => {
                println!("Unsuccessful drawing: {s}")
            }
        }
        renderer.present_canvas();
    }
}

pub fn model_to_map_coordinate(x: f32, y: f32, gamestate_scale: u16) -> sdl2::rect::Point {
    let x_screen = x / (gamestate_scale as f32);
    let y_screen = y / (gamestate_scale as f32);

    sdl2::rect::Point::new((x_screen * 35.0) as i32, (y_screen * 35.0) as i32)
}
