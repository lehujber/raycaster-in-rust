mod player;
use crate::player::Player;
#[derive(Debug)]
#[allow(dead_code)]
pub struct Gamestate {
    map: Vec<Vec<bool>>,
    player: Player,
}

impl Gamestate {
    pub fn new(map: Vec<Vec<bool>>, player_x: i32, player_y: i32) -> Gamestate {
        let game_map = map;
        let player = player::Player::new(player_x, player_y, 120.0);
        Gamestate {
            map: game_map,
            player,
        }
    }

    pub fn map(&self) -> &Vec<Vec<bool>> {
        &self.map
    }
}
