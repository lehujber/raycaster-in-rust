#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    movement_speed: (f32, f32),
    view_direction: f32,
    field_of_view: f32,
    posisiton: (f32, f32),
    view_distance: i32,
}

impl Player {
    pub fn new(x_position: f32, y_position: f32, fov: f32) -> Player {
        Player {
            movement_speed: (0.0, 0.0),
            view_direction: 90.0,
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
    pub fn update_position(&mut self, elpased_milliseconds: f32, movement_speed: f32) {
        const RADIAN_MULTIPLIER: f32 = std::f32::consts::PI / 180.0;

        let delta_time = elpased_milliseconds / 1000.0;
        let (x_pos, y_pos) = self.posisiton;

        let y_weight = (self.view_direction * RADIAN_MULTIPLIER).sin();
        let x_weight = (self.view_direction * RADIAN_MULTIPLIER).cos();

        // let (x_speed, y_speed) = self.movement_speed;
        self.posisiton = (
            x_pos - x_weight * delta_time * movement_speed,
            y_pos - y_weight * delta_time * movement_speed,
        )
    }

    pub fn view_direction(&self) -> f32 {
        self.view_direction
    }
    pub fn rotate(&mut self, rotation_val: f32) {
        self.view_direction += rotation_val;
        if self.view_direction < 0.0 {
            self.view_direction += 360.0;
        } else if self.view_direction > 360.0 {
            self.view_direction -= 360.0;
        }
        let view_dir = self.view_direction;
        println!("View direction: {view_dir}")
    }
}
