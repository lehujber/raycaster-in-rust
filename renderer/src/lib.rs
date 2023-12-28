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

    pub fn draw_square(&mut self, middle: &sdl2::rect::Point, width: u32) -> Result<(), String> {
        self.canvas.set_draw_color(self.foreground_color);
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(middle.x, middle.y, width, width))
    }

    pub fn draw_map(&mut self, map: &Vec<Vec<bool>>, block_size: u32) -> Result<(), String> {
        self.canvas.set_draw_color(self.foreground_color);
        let map_width = map.get(0).unwrap().len() as u8;

        use sdl2::rect::Rect;
        let nums = std::iter::successors(Some(0_u8), |x| x.checked_add(1));
        let walled_spaces = map
            .iter()
            .flatten()
            .zip(nums)
            .filter(|x| *(x.0))
            .map(|x| (x.1 % map_width, x.1 / map_width))
            .map(|(x, y)| {
                Rect::new(
                    (x as i32) * (block_size as i32),
                    (y as i32) * (block_size as i32),
                    block_size,
                    block_size,
                )
            })
            .collect::<Vec<Rect>>();

        self.canvas.fill_rects(&walled_spaces)
    }
}
