use crate::coord::Coord;

#[derive(Copy, Clone, PartialEq, Eq, Hash )]
pub struct Segment {
    pub start: Coord,
    pub end: Coord,
}
impl Segment {
    pub fn new(a: Coord, b: Coord) -> Segment {
        if a < b {
            Segment { start: a, end: b }
        } else {
            Segment { start: b, end: a }
        }
    }
    pub fn contains(self, coord: Coord) -> bool {
        coord.x >= self.start.x && coord.x <= self.end.x && coord.y >= self.start.y && coord.y <= self.end.y
    }
}
impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        self.start.fmt(f).unwrap();
        write!(f, " -> ").unwrap();
        self.end.fmt(f)
    }
}