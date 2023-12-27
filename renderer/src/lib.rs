#[allow(dead_code)]
pub struct Renderer {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,

    background_color: sdl2::pixels::Color,
    foreground_color: sdl2::pixels::Color,
}

impl Renderer {
    pub fn new(width: u32, height: u32, title: &str) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        let background_color = sdl2::pixels::Color::RGB(0, 0, 0);
        let foreground_color = sdl2::pixels::Color::RGB(0, 0, 0);
        Renderer {
            sdl_context,
            video_subsystem,
            canvas,
            background_color,
            foreground_color,
        }
    }

    pub fn set_background(&mut self, color: sdl2::pixels::Color) {
        self.background_color = color;
    }
    pub fn set_foreground(&mut self, color: sdl2::pixels::Color) {
        self.foreground_color = color;
    }
    pub fn present_canvas(&mut self) {
        self.canvas.present();
    }
    pub fn clear_canvas(&mut self) {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
    }
    pub fn event_pump(&self) -> sdl2::EventPump {
        self.sdl_context.event_pump().unwrap()
    }
    pub fn draw_line(
        &mut self,
        start: &sdl2::rect::Point,
        end: &sdl2::rect::Point,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(self.foreground_color);
        self.canvas.draw_line(*start, *end)
    }

    pub fn draw_square(&mut self, middle: &sdl2::rect::Point, width: i32) -> Result<(), String> {
        self.canvas.set_draw_color(self.foreground_color);

        use sdl2::rect::Point;

        let x = middle.x;
        let y = middle.y;
        let offset = width / 2;
        let points: &[Point] = &[
            Point::new(x - offset, y - offset),
            Point::new(x - offset, y + offset),
            Point::new(x + offset, y + offset),
            Point::new(x + offset, y - offset),
            Point::new(x - offset, y - offset),
        ];

        self.canvas.draw_lines(points)
    }
}
