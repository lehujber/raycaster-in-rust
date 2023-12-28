#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    movement_speed: (f32, f32),
    view_direction: (f32, f32),
    field_of_view: f32,
    posisiton: (f32, f32),
    view_distance: i32,
}

impl Player {
    pub fn new(x_position: f32, y_position: f32, fov: f32) -> Player {
        Player {
            movement_speed: (0.0, 0.0),
            view_direction: (0.0, 0.0),
            field_of_view: fov,
            posisiton: (x_position, y_position),
            view_distance: 40,
        }
    }

    pub fn position(&self) -> (f32, f32) {
        self.posisiton
    }
    pub fn update_speed(&mut self, speed: (f32, f32)) {
        self.movement_speed = speed
    }
    pub fn update_position(&mut self, elpased_milliseconds: f32) {
        let delta_time = elpased_milliseconds / 1000.0;
        let (x_pos, y_pos) = self.posisiton;
        let (x_speed, y_speed) = self.movement_speed;
        self.posisiton = (x_pos - x_speed * delta_time, y_pos - y_speed * delta_time)
    }
}
