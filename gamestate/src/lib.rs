mod map;
mod player;
use std::env::VarError;

use crate::map::Map;
use crate::player::Player;
#[allow(dead_code)]
pub struct Gamestate {
    map: Map,
    player: Player,
    block_size: u16,
    ray_angles: Vec<f32>,
}

impl Gamestate {
    const PLAYER_FOV: f32 = 120.0;
    pub fn new(
        map_matrix: Vec<Vec<bool>>,
        player_x: f32,
        player_y: f32,
        block_size: u16,
        ray_count: u16,
    ) -> Gamestate {
        let map = Map::new(map_matrix);
        let player = player::Player::new(player_x, player_y, Gamestate::PLAYER_FOV, 150);

        let ray_angles = (1..)
            .map(|v| {
                (Gamestate::PLAYER_FOV / (ray_count + 1) as f32) * (v as f32)
                    + (Gamestate::PLAYER_FOV)
            })
            .take(ray_count as usize)
            .collect::<Vec<f32>>();

        Gamestate {
            map,
            player,
            block_size,
            ray_angles,
        }
    }

    pub fn map_walls(&self) -> &Vec<u16> {
        self.map.walls()
    }
    pub fn map_width(&self) -> u8 {
        self.map.width()
    }
    pub fn map_height(&self) -> u8 {
        self.map.height()
    }
    pub fn block_size(&self) -> u16 {
        self.block_size
    }

    pub fn player_position(&self) -> (f32, f32) {
        self.player.position()
    }

    pub fn player_rotation(&self) -> f32 {
        self.player.view_direction()
    }

    pub fn view_distance(&self) -> f32 {
        self.player.view_distance() as f32
    }
    pub fn player_rotate(&mut self, dir: TurnDirection, delta_time: u128) {
        match dir {
            TurnDirection::Left => self.player.rotate(-1.0, delta_time),
            TurnDirection::Right => self.player.rotate(1.0, delta_time),
        }
    }
    pub fn player_move(&mut self, dir: MoveDirection, delta_time: u128) {
        let (x_past, y_past) = self.player_position();

        match dir {
            MoveDirection::Forward => self.player.update_position(delta_time, 1.0),
            MoveDirection::Backward => self.player.update_position(delta_time, -1.0),
        }

        if !self.valdate_position() {
            let (x_curr, y_curr) = self.player_position();

            let (x_block_past, y_block_past) = self.imaginary_block_position(x_past, y_past);

            let (x_block_curr, y_block_curr) = self.imaginary_block_position(x_curr, y_curr);

            let row_step_direction = if x_curr < 0.0 {
                -1
            } else {
                (x_block_curr - x_block_past).clamp(-1, 1)
            };
            let col_step_direction = if y_curr < 0.0 {
                -1
            } else {
                (y_block_curr - y_block_past).clamp(-1, 1)
            };

            let (x_left, y_top, x_right, y_bottom) =
                self.block_corners(self.block_id(x_past, y_past));

            let m = (y_past - y_curr).abs() / (x_past - x_curr).abs();
            let b = y_past - (m * x_past);

            let new_cords_x = match row_step_direction {
                1 => Some((x_right, m * x_right + b)),
                -1 => Some((x_left, m * x_left + b)),
                _ => None,
            };

            let new_cords_y = match col_step_direction {
                1 => Some(((y_bottom - b) / m, y_bottom)),
                -1 => Some(((y_top - b) / m, y_top)),
                _ => None,
            };

            match (new_cords_x, new_cords_y) {
                (None, None) => {
                    println!("No solution found, resetting to previous position");
                    self.player.set_position(x_past, y_past);
                }
                (Some(cords), None) | (None, Some(cords)) => {
                    let (x, y) = cords;
                    self.player.set_position(x, y);
                }
                (Some(x_cords), Some(y_cords)) => {
                    if self.imaginary_block_position(x_cords.0, x_cords.1)
                        == (x_block_past, y_block_past)
                    {
                        self.player.set_position(x_cords.0, x_cords.1);
                    } else if self.imaginary_block_position(y_cords.0, y_cords.1)
                        == (x_block_past, y_block_past)
                    {
                        self.player.set_position(y_cords.0, y_cords.1);
                    } else {
                        println!("No solution found, resetting to previous position");
                        self.player.set_position(x_past, y_past);
                    }
                }
            }
        }
    }

    pub fn cast_rays(&self) -> Vec<(f32, f32, bool)> {
        const RADIAN_MULTIPLIER: f32 = std::f32::consts::PI / 180.0;
        let player_angle = self.player.view_direction() * RADIAN_MULTIPLIER;

        self.ray_angles
            .iter()
            .map(|ray_angle| {
                let angle = player_angle - (ray_angle * RADIAN_MULTIPLIER);

                self.ray_wall_collision(angle)
            })
            .collect::<Vec<(f32, f32, bool)>>()
    }

    fn ray_wall_collision(&self, ray_angle: f32) -> (f32, f32, bool) {
        const RADIAN_MULTIPLIER: f32 = std::f32::consts::PI / 180.0;
        let view_distance = self.player.view_distance() as f32;
        let (player_x, player_y) = self.player.position();
        let (sin, cos) = ray_angle.sin_cos();

        let end_x = cos * view_distance + player_x;
        let end_y = sin * view_distance + player_y;

        let atan = (end_y - player_y).atan2(end_x - player_x) / RADIAN_MULTIPLIER + 180.0;
        let quartet = (atan / (360.0 / 4.0)) as i32;

        enum CordDir {
            Positive,
            Negative,
        }

        let (hor, vert) = match quartet {
            0 => (CordDir::Negative, CordDir::Positive),
            1 => (CordDir::Positive, CordDir::Positive),
            3 => (CordDir::Positive, CordDir::Negative),
            _ => (CordDir::Negative, CordDir::Negative),
        };

        fn smaller_distance(
            (cos, sin): (f32, f32),
            (x_fix, y_fix): (f32, f32),
            (x, y): (f32, f32),
        ) -> f32 {
            let dist_to_hor = ((y_fix - y) / sin).abs();
            let dist_to_vert = ((x_fix - x) / cos).abs();
            dist_to_hor.min(dist_to_vert)
        }

        let mut dist = 0.0;
        // (cos * dist + player_x, sin * dist + player_y)
        while dist < view_distance {
            let x_next = cos * (dist + 5.0) + player_x;
            let y_next = sin * (dist + 5.0) + player_y;
            if self.map_walls().contains(&self.block_id(x_next, y_next)) {
                return (cos * dist + player_x, sin * dist + player_y, true);
            }

            let (neg_x, pos_y, pos_x, neg_y) = self.block_corners(self.block_id(x_next, y_next));
            let new_dist = match (&hor, &vert) {
                (CordDir::Negative, CordDir::Positive) => {
                    smaller_distance((cos, sin), (neg_x, pos_y), (player_x, player_y))
                }
                (CordDir::Positive, CordDir::Positive) => {
                    smaller_distance((cos, sin), (pos_x, pos_y), (player_x, player_y))
                }
                (CordDir::Positive, CordDir::Negative) => {
                    smaller_distance((cos, sin), (neg_x, neg_y), (player_x, player_y))
                }
                (CordDir::Negative, CordDir::Negative) => {
                    smaller_distance((cos, sin), (pos_x, neg_y), (player_x, player_y))
                }
            };

            if new_dist == dist {
                return (cos * dist + player_x, sin * dist + player_y, true);
            }
            dist = new_dist;
        }

        (
            cos * view_distance + player_x,
            sin * view_distance + player_y,
            false,
        )
    }

    fn valdate_position(&self) -> bool {
        let (x, y) = self.player.position();
        let (w, h) = (self.map.width() as f32, self.map.height() as f32);

        if x > w * self.block_size as f32 || x < 0.0 || y > h * self.block_size as f32 || y < 0.0 {
            return false;
        }

        !self.map.walls().contains(&self.block_id(x, y))
    }

    fn block_id(&self, x: f32, y: f32) -> u16 {
        let x_block = (x / self.block_size as f32) as u16;
        let y_block = (y / self.block_size as f32) as u16;

        y_block * self.map_width() as u16 + x_block
    }

    fn block_corners(&self, block_id: u16) -> (f32, f32, f32, f32) {
        let y_block = block_id / self.map_width() as u16;
        let x_block = block_id % self.map_width() as u16;

        let (x_top, y_top) = (x_block * self.block_size, y_block * self.block_size);
        let (x_bottom, y_bottom) = (x_top + self.block_size - 1, y_top + self.block_size - 1);
        (x_top as f32, y_top as f32, x_bottom as f32, y_bottom as f32)
    }

    fn imaginary_block_position(&self, x: f32, y: f32) -> (i16, i16) {
        let block_x = x as i16 / self.block_size as i16;
        let block_y = y as i16 / self.block_size as i16;

        (block_x, block_y)
    }
}

pub enum TurnDirection {
    Left,
    Right,
}
pub enum MoveDirection {
    Forward,
    Backward,
}
