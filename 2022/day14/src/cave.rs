use crate::{segment::Segment, coord::Coord};

#[derive(Clone, PartialEq, Eq, Hash, Debug )]
pub struct Cave {
    pub segments: Vec<Segment>,
    pub sands: Vec<Coord>
}
impl Cave {
    pub fn fill_sand(&mut self, start: Coord) -> bool {
        if start.y == self.segments.iter().map(|segment| segment.end.y).max().unwrap() {
            return false;
        }
        let next_sands = start
            .move_positions()
            .into_iter()
            .filter(|sand| self.is_free(*sand) )
            .collect::<Vec<Coord>>();
        if next_sands.is_empty() {
            let pos = self.sands.binary_search(&start).unwrap_or_else(|e| e);
            self.sands.insert(pos, start);
            return true;
        }
        let mut result = true;
        for next_sand in next_sands {
            result = result && self.fill_sand(next_sand);
        }
        if result {
            let pos = self.sands.binary_search(&start).unwrap_or_else(|e| e);
            self.sands.insert(pos, start);
        }
        result
    }
    pub fn is_free(&self, sand: Coord) -> bool {
        !self.segments.iter().map(|segment: &Segment | segment.contains(sand)).any(|b| b)
        && self.sands.binary_search(&sand).is_err()
    }
}
