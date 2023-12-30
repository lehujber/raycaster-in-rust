#[allow(dead_code)]
pub struct Map {
    width: u8,
    height: u8,
    walls: Vec<u16>,
}

impl Map {
    pub fn new(map: Vec<Vec<bool>>) -> Map {
        let height = map.len() as u8;
        let width = map.get(0).unwrap().len() as u8;

        let walls = map
            .iter()
            .flatten()
            .zip(0..)
            .filter(|(x, _)| **x)
            .map(|(_, y)| y)
            .collect::<Vec<u16>>();

        Map {
            width,
            height,
            walls,
        }
    }

    pub fn walls(&self) -> &Vec<u16> {
        &self.walls
    }
    pub fn width(&self) -> u8 {
        self.width
    }
    pub fn height(&self) -> u8 {
        self.height
    }
}
