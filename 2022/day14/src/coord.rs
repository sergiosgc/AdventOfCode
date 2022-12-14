use std::cmp::Ordering;

use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, Hash )]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64) -> Coord { Coord{ x, y } }
    pub fn from_string(s: &str) -> Coord {
        let captures = Regex::new(r#"^ *(?P<x>\d+) *, *(?P<y>\d+)"#).unwrap().captures(s).unwrap();
        match (captures.name("x"), captures.name("y")) {
            (Some(x), Some(y)) => Coord::new(x.as_str().parse::<i64>().unwrap(), y.as_str().parse::<i64>().unwrap()),
            _ => panic!("Unable to parse {}", s)
        }
    }
    pub fn move_positions(self) -> Vec::<Coord> {
        [
            Coord::new(self.x, self.y + 1),
            Coord::new(self.x - 1, self.y + 1),
            Coord::new(self.x + 1, self.y+ 1),
        ].to_vec()
    }
}
impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({},{})", self.x, self.y) }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.cmp(&other.x) {
            Ordering::Equal => { Some(self.y.cmp(&other.y)) },
            result => Some(result)

        }
    }
}
impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => { self.y.cmp(&other.y) },
            result => result
        }
    }
}