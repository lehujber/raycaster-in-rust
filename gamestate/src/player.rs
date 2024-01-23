#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    movement_speed: f32,
    view_direction: f32,
    field_of_view: f32,
    posisiton: (f32, f32),
    view_distance: u16,
}

impl Player {
    pub fn new(x_position: f32, y_position: f32, fov: f32, view_distance: u16) -> Player {
        Player {
            movement_speed: 0.0000002,
            view_direction: 90.0,
            field_of_view: fov,
            posisiton: (x_position, y_position),
            view_distance,
        }
    }

    pub fn position(&self) -> (f32, f32) {
        self.posisiton
    }
    pub fn update_position(&mut self, elpased_nanoseconds: u128, movement_offset: f32) {
        const RADIAN_MULTIPLIER: f32 = std::f32::consts::PI / 180.0;

        let delta_time = elpased_nanoseconds;
        let (x_pos, y_pos) = self.posisiton;

        let y_weight = (self.view_direction * RADIAN_MULTIPLIER).sin();
        let x_weight = (self.view_direction * RADIAN_MULTIPLIER).cos();

        // let (x_speed, y_speed) = self.movement_speed;
        self.posisiton = (
            x_pos - x_weight * (delta_time as f32) * self.movement_speed * movement_offset,
            y_pos - y_weight * (delta_time as f32) * self.movement_speed * movement_offset,
        )
    }

    pub fn view_direction(&self) -> f32 {
        self.view_direction
    }
    pub fn rotate(&mut self, rotation_val: f32, delta_time: u128) {
        self.view_direction += rotation_val * (delta_time as f32) * 0.0000004;
        if self.view_direction < 0.0 {
            self.view_direction += 360.0;
        } else if self.view_direction > 360.0 {
            self.view_direction -= 360.0;
        }
    }
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.posisiton = (x, y);
    }
    pub fn view_distance(&self) -> u16 {
        self.view_distance
    }
}
