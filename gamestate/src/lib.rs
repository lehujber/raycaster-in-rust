mod map;
mod player;
use crate::map::Map;
use crate::player::Player;
#[derive(Debug)]
#[allow(dead_code)]
struct Gamestate {
    map: Map,
    player: Player,
}

impl Gamestate {
    pub fn new(map: Map, player: Player) -> Gamestate {
        Gamestate { map, player }
    }
}
