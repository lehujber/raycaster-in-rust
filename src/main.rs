use gamestate::Gamestate;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::f32;

const RENDER_BLOCK_SIZE: u32 = 70;

pub fn main() {
    let mut renderer = renderer::Renderer::new(800, 600, "Sdl demo window");

    let map = [
        [true, false, false, false],
        [true, false, true, false],
        [true, false, true, false],
        [true, false, false, false],
    ]
    .iter()
    .map(|row| Vec::from(row))
    .collect::<Vec<Vec<bool>>>();

    let mut gamestate = gamestate::Gamestate::new(map, 150.0, 150.0, 100);

    renderer.set_background_color(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.set_wall_color(sdl2::pixels::Color::RGB(67, 255, 20));
    renderer.set_floor_color(sdl2::pixels::Color::RGB(255, 0, 255));
    renderer.set_player_color(sdl2::pixels::Color::RGB(0, 0, 255));

    match renderer.set_scale(2.0) {
        Ok(_) => {}
        Err(s) => {
            println!("Error setting scale: {s}")
        }
    }

    let mut event_pump = renderer.event_pump();

    'running: loop {
        renderer.clear_canvas();
        let map_drawing_res = renderer.draw_map(
            gamestate.map_walls(),
            gamestate.map_width(),
            gamestate.map_height(),
            70,
        );
        match map_drawing_res {
            Ok(_) => {}
            Err(s) => {
                println!("Unsuccessful drawing: {s}")
            }
        }

        let (x, y) = gamestate.player_position();
        let player_drawing_res = renderer.draw_player(
            model_to_screen_coordinate(x, y, gamestate.block_size()),
            gamestate.player_rotation(),
        );
        match player_drawing_res {
            Ok(_) => {}
            Err(s) => {
                println!("Unsuccessful drawing: {s}")
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: code, .. } => match code {
                    Some(Keycode::W) => {
                        println!("W pressed");
                        gamestate.player_move(gamestate::MoveDirection::Forward, 1000.0)
                    }
                    Some(Keycode::S) => {
                        println!("S pressed");
                        gamestate.player_move(gamestate::MoveDirection::Backward, 1000.0)
                    }
                    Some(Keycode::A) => {
                        println!("A pressed");
                        gamestate.player_rotate(gamestate::TurnDirection::Left)
                    }
                    Some(Keycode::D) => {
                        println!("D pressed");
                        gamestate.player_rotate(gamestate::TurnDirection::Right)
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        renderer.present_canvas();
    }
}

pub fn model_to_screen_coordinate(x: f32, y: f32, gamestate_scale: u8) -> sdl2::rect::Point {
    let x_screen = x / (gamestate_scale as f32);
    let y_screen = y / (gamestate_scale as f32);

    sdl2::rect::Point::new((x_screen * 70.0) as i32, (y_screen * 70.0) as i32)
}
