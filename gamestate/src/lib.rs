mod map;
mod player;
use crate::map::Map;
use crate::player::Player;
#[allow(dead_code)]
pub struct Gamestate {
    map: Map,
    player: Player,
}

impl Gamestate {
    pub fn new(map_matrix: Vec<Vec<bool>>, player_x: f32, player_y: f32) -> Gamestate {
        let map = Map::new(map_matrix);
        let player = player::Player::new(player_x, player_y, 120.0);
        Gamestate { map, player }
    }

    pub fn map_walls(&self) -> &Vec<u8> {
        self.map.walls()
    }
    pub fn map_width(&self) -> u8 {
        self.map.width()
    }
    pub fn map_height(&self) -> u8 {
        self.map.height()
    }

    pub fn player_position(&self) -> (f32, f32) {
        self.player.position()
    }

    pub fn player_rotation(&self) -> f32 {
        self.player.view_direction()
    }
    pub fn player_rotate(&mut self, dir: TurnDirection) {
        match dir {
            TurnDirection::Left => self.player.rotate(-10.0),
            TurnDirection::Right => self.player.rotate(10.0),
        }
    }
    pub fn player_move(&mut self, dir: MoveDirection, delta_time: f32) {
        match dir {
            MoveDirection::Forward => self.player.update_position(delta_time, 10.0),
            MoveDirection::Backward => self.player.update_position(delta_time, -10.0),
        }
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
