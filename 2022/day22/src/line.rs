use crate::coord::Coord;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Line {
    pub from: Coord,
    pub to: Coord,
}
impl Line {
    pub fn contains(&self, coord: &Coord) -> bool {
        self.from.x == self.to.x && self.from.x == coord.x && coord.y >= self.from.y.min(self.to.y) && coord.y <= self.from.y.max(self.to.y) || 
        self.from.y == self.to.y && self.from.y == coord.y && coord.x >= self.from.x.min(self.to.x) && coord.x <= self.from.x.max(self.to.x)
    }
    pub fn coords(self) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();
        if self.from.x == self.to.x {
            for y in self.from.y.min(self.to.y)..self.from.y.max(self.to.y)+1 {
                result.push(Coord{ x: self.from.x, y });
            }
        } else {
            for x in self.from.x.min(self.to.x)..self.from.x.max(self.to.x)+1 {
                result.push(Coord{ x, y: self.from.y });
            }
        }
        if self.from.x > self.to.x || self.from.y > self.to.y {
            result.reverse();
        }

        result
    }
}
