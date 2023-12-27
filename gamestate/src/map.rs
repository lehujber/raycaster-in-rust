#[derive(Debug)]
#[allow(dead_code)]
pub struct Map {
    map: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(map: Vec<Vec<bool>>) -> Map {
        Map { map }
    }
}
