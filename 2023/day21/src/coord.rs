use std::ops::{Add, Neg, Sub, Mul};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64) -> Coord { Coord{ x, y } }
    pub fn neighbours(&self) -> Vec::<Coord> {
        vec![
            *self + Coord { x: 0, y: -1},
            *self + Coord { x: -1, y: 0},
            *self + Coord { x: 1, y: 0},
            *self + Coord { x: 0, y: 1},
        ].to_vec()
    }
    pub fn bounding_min(a: &Coord, b: &Coord) -> Coord {
        Coord::new(a.x.min(b.x), a.y.min(b.y))
    }
    pub fn bounding_max(a: &Coord, b: &Coord) -> Coord {
        Coord::new(a.x.max(b.x), a.y.max(b.y))
    }
    pub fn inside(self, a: &Coord, b: &Coord) -> bool {
        self.x >= a.x.min(b.x)
        && self.x <= a.x.max(b.x)
        && self.y >= a.y.min(b.y)
        && self.y <= a.y.max(b.y)
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
impl Mul<i64> for Coord {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Coord { x: self.x * rhs, y: self.y * rhs }
    }
}
impl From<(i64, i64)> for Coord {
    fn from((x, y): (i64, i64)) -> Self {
        Coord { x, y }
    }
}
impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord::from((x as i64, y as i64))
    }
}
impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord::from((x as i64, y as i64))
    }
}
impl From<Coord> for (i64, i64) {
    fn from(value: Coord) -> Self {
        (value.x, value.y)
    }
}
impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> Self {
        (value.x.try_into().unwrap(), value.y.try_into().unwrap())
    }
}
impl From<&Coord> for (i64, i64) {
    fn from(value: &Coord) -> Self {
        (value.x, value.y)
    }
}
impl From<&Coord> for (usize, usize) {
    fn from(value: &Coord) -> Self {
        (value.x.try_into().unwrap(), value.y.try_into().unwrap())
    }
}