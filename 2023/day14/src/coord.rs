use std::{ops::{Add, Neg, Sub}};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64) -> Coord { Coord{ x, y } }
    pub fn neighbours(self) -> Vec::<Coord> {
        vec![
            self + Coord { x: 0, y: -1},
            self + Coord { x: -1, y: 0},
            self + Coord { x: 1, y: 0},
            self + Coord { x: 0, y: 1},
        ].to_vec()
    }
    pub fn bounding_min(a: &Coord, b: &Coord) -> Coord {
        Coord::new(a.x.min(b.x), a.y.min(b.y))
    }
    pub fn bounding_max(a: &Coord, b: &Coord) -> Coord {
        Coord::new(a.x.max(b.x), a.y.max(b.y))
    }
}
impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Neg for Coord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x, 
            y: -self.y,
        }
    }
}