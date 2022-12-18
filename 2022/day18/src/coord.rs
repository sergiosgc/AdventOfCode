use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64, z: i64) -> Coord { Coord{ x, y, z } }
    pub fn neighbours(self) -> Vec::<Coord> {
        vec![
            self + Coord { x: 1, y: 0, z: 0},
            self + Coord { x: -1, y: 0, z: 0},
            self + Coord { x: 0, y: 1, z: 0},
            self + Coord { x: 0, y: -1, z: 0},
            self + Coord { x: 0, y: 0, z: 1},
            self + Coord { x: 0, y: 0, z: -1},
        ].to_vec()
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}