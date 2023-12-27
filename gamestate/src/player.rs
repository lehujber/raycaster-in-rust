#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    movement_speed: (i32, i32),
    view_direction: (f32, f32),
    field_of_view: f32,
    posisiton: (i32, i32),
}

impl Player {
    pub fn new(x_position: i32, y_position: i32, fov: f32) -> Player {
        Player {
            movement_speed: (0, 0),
            view_direction: (0.0, 0.0),
            field_of_view: fov,
            posisiton: (x_position, y_position),
        }
    }
}
