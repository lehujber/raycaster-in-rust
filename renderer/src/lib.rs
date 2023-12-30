#[allow(dead_code)]
pub struct Renderer {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,

    background_color: sdl2::pixels::Color,
    wall_color: sdl2::pixels::Color,
    floor_color: sdl2::pixels::Color,
    player_color: sdl2::pixels::Color,
    ray_color: sdl2::pixels::Color,
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
        let wall_color = sdl2::pixels::Color::RGB(0, 0, 0);
        let floor_color = sdl2::pixels::Color::RGB(0, 0, 0);
        let player_color = sdl2::pixels::Color::RGB(0, 0, 0);
        let ray_color = sdl2::pixels::Color::RGB(0, 0, 0);

        Renderer {
            sdl_context,
            video_subsystem,
            canvas,
            background_color,
            wall_color,
            floor_color,
            player_color,
            ray_color,
        }
    }

    pub fn set_background_color(&mut self, color: sdl2::pixels::Color) {
        self.background_color = color;
    }
    pub fn set_wall_color(&mut self, color: sdl2::pixels::Color) {
        self.wall_color = color;
    }
    pub fn set_floor_color(&mut self, color: sdl2::pixels::Color) {
        self.floor_color = color
    }
    pub fn set_player_color(&mut self, color: sdl2::pixels::Color) {
        self.player_color = color
    }
    pub fn set_ray_color(&mut self, color: sdl2::pixels::Color) {
        self.ray_color = color
    }
    pub fn set_scale(&mut self, scale: f32) -> Result<(), String> {
        self.canvas.set_scale(scale, scale)
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
        self.canvas.set_draw_color(self.wall_color);
        self.canvas.draw_line(*start, *end)
    }

    pub fn draw_square(&mut self, middle: &sdl2::rect::Point, width: u32) -> Result<(), String> {
        self.canvas.set_draw_color(self.wall_color);
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(middle.x, middle.y, width, width))
    }

    pub fn draw_map(
        &mut self,
        map: &[u8],
        map_width: u8,
        map_height: u8,
        block_size: u32,
    ) -> Result<(), String> {
        use sdl2::rect::Rect;

        self.canvas.set_draw_color(self.floor_color);
        let drawing_res = self.canvas.fill_rect(Rect::new(
            (map_width / 2) as i32 - 1,
            (map_height / 2) as i32 - 1,
            map_width as u32 * block_size,
            map_height as u32 * block_size,
        ));
        match drawing_res {
            Ok(_) => {}
            Err(_) => return drawing_res,
        }

        self.canvas.set_draw_color(self.wall_color);
        let walled_spaces = map
            .iter()
            .map(|x| (x % map_width, x / map_width))
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

    pub fn draw_player(
        &mut self,
        position: sdl2::rect::Point,
        player_rotation: f32,
    ) -> Result<(), String> {
        use sdl2::rect::Point;
        self.canvas.set_draw_color(self.player_color);

        let x = position.x;
        let y = position.y;
        let drawing_res = self.canvas.draw_line(
            Renderer::rotate_point(Point::new(x, y + 3), position, player_rotation),
            Renderer::rotate_point(Point::new(x, y - 3), position, player_rotation),
        );
        match drawing_res {
            Ok(_) => {}
            Err(_) => return drawing_res,
        }

        self.canvas.draw_line(
            Renderer::rotate_point(Point::new(x - 5, y), position, player_rotation),
            Renderer::rotate_point(Point::new(x + 2, y), position, player_rotation),
        )
    }

    pub fn draw_rays(
        &mut self,
        player_position: sdl2::rect::Point,
        ray_targets: Vec<sdl2::rect::Point>,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(self.ray_color);

        for ray_target in ray_targets {
            match self.canvas.draw_line(player_position, ray_target) {
                Ok(_) => {}
                Err(s) => return Err(s),
            };
        }

        Result::Ok(())
    }

    fn rotate_point(
        p: sdl2::rect::Point,
        center: sdl2::rect::Point,
        rotation: f32,
    ) -> sdl2::rect::Point {
        use sdl2::rect::Point;

        const RADIAN_MULTIPLIER: f32 = std::f32::consts::PI / 180.0;

        let rotation_rad = rotation * RADIAN_MULTIPLIER;

        let s = (rotation_rad).sin();
        let c = (rotation_rad).cos();

        let t = p - center;

        let nx = ((t.x as f32) * c - (t.y as f32) * s) + (center.x as f32);
        let ny = ((t.x as f32) * s + (t.y as f32) * c) + (center.y as f32);

        Point::new(nx as i32, ny as i32)
    }
}
