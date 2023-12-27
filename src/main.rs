use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

pub fn main() {
    let mut renderer = renderer::Renderer::new(800, 600, "Sdl demo window");

    renderer.set_background(sdl2::pixels::Color::RGB(0, 0, 0));
    renderer.set_foreground(sdl2::pixels::Color::RGB(67, 255, 20));

    let mut event_pump = renderer.event_pump();

    'running: loop {
        renderer.clear_canvas();

        let drawing_res = renderer.draw_square(&Point::new(20, 20), 30);
        match drawing_res {
            Ok(_) => {}
            Err(_) => {
                println!("Unsuccessful drawing")
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        renderer.present_canvas();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
