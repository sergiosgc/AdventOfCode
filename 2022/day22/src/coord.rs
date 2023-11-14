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
            self + Coord { x: -1, y: -1},
            self + Coord { x: 1, y: -1},
            self + Coord { x: -1, y: 1},
            self + Coord { x: 1, y: 1},
        ].to_vec()
    }
    pub fn levenshtein_distance(self, other: &Coord) -> i64 {
        (other.x - self.x).abs() + 
        (other.y - self.y).abs()
    }
}
pub trait LineSortable {
    fn sort_into_line(self, starting_at: Coord) -> Self;
}
impl LineSortable for Vec<Coord> {
    fn sort_into_line(self, starting_at: Coord) -> Vec<Coord> {
        let mut unsorted = self.clone();
        let mut current = starting_at;
        let mut sorted: Vec<Coord> = vec![current];
        {
            let pos = unsorted.iter().position(|candidate| *candidate == current).unwrap();
            unsorted.remove(pos);
        }
        while !unsorted.is_empty() {
            let pos = unsorted.iter().position(|candidate| current.levenshtein_distance(candidate) == 1 ).unwrap();
            current = unsorted.remove(pos);
            sorted.push(current);
        }

        sorted
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