mod map;
mod player;
use crate::map::Map;
use crate::player::Player;
#[allow(dead_code)]
pub struct Gamestate {
    map: Map,
    player: Player,
    block_size: u8,
}

impl Gamestate {
    pub fn new(
        map_matrix: Vec<Vec<bool>>,
        player_x: f32,
        player_y: f32,
        block_size: u8,
    ) -> Gamestate {
        let map = Map::new(map_matrix);
        let player = player::Player::new(player_x, player_y, 120.0);
        Gamestate {
            map,
            player,
            block_size,
        }
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
    pub fn block_size(&self) -> u8 {
        self.block_size
    }

    pub fn player_position(&self) -> (f32, f32) {
        self.player.position()
    }

    pub fn player_rotation(&self) -> f32 {
        self.player.view_direction()
    }
    pub fn player_rotate(&mut self, dir: TurnDirection, delta_time: u128) {
        match dir {
            TurnDirection::Left => self.player.rotate(-1.0, delta_time),
            TurnDirection::Right => self.player.rotate(1.0, delta_time),
        }
    }
    pub fn player_move(&mut self, dir: MoveDirection, delta_time: u128) {
        let (x_past, y_past) = self.player_position();
        // println!("past pos: {x_past}, {y_past}");

        match dir {
            MoveDirection::Forward => self.player.update_position(delta_time, 1.0),
            MoveDirection::Backward => self.player.update_position(delta_time, -1.0),
        }

        if !self.valdate_position() {
            self.player.set_position(x_past, y_past);
        } else {
            // println!("cant move player :(");
        }
    }

    fn valdate_position(&self) -> bool {
        let (x, y) = self.player.position();
        let (w, h) = (self.map.width() as f32, self.map.height() as f32);

        if x > w * self.block_size as f32 || x < 0.0 || y > h * self.block_size as f32 || y < 0.0 {
            return false;
        }

        !self.map.walls().contains(&self.block_id(x, y))
    }

    fn block_id(&self, x: f32, y: f32) -> u8 {
        let x_block = (x / self.block_size as f32) as u8;
        let y_block = (y / self.block_size as f32) as u8;

        y_block * self.map_width() + x_block
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
