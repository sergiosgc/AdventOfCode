use crate::coord::Coord;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug )]
pub struct Segment {
    pub start: Coord,
    pub end: Coord
}
impl Segment {
    pub fn new(a: Coord, b: Coord) -> Segment {
        if a < b { Segment{ start: a, end: b } } else { Segment{ start: b, end: a }}
    }
    pub fn length(self) -> i64 {
        self.end.x - self.start.x + 1
    }
    pub fn subtract(self, other: &Segment) -> (Option<Segment>, Option<Segment>) {
        (Segment {
            start: Coord { x: self.start.x, y: self.start.y },
            end: Coord { x: other.start.x - 1, y: self.start.y },
        }.valid(),Segment {
            start: Coord { x: other.end.x + 1, y: self.start.y },
            end: Coord { x: self.end.x, y: self.start.y },
        }.valid())
    }
    pub fn valid(self) -> Option<Segment> {
        if self.start.x <= self.end.x && self.start.y <= self.end.y { Some(self) } else { None }
    }
    pub fn overlaps(self, other: Segment) -> bool {
        !(self.start.x > other.end.x || self.end.x < other.start.x)
    }
}
pub trait InsertNonOverlapping {
    fn insert_non_overlapping(self, egment: Segment) -> Self;
}
impl InsertNonOverlapping for Vec<Segment> {
    fn insert_non_overlapping(mut self, segment: Segment) -> Vec<Segment> {
        let mut queue: Vec<Segment> = vec![segment];
        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            let other = self.iter().find(|other| other.overlaps(current) );
            match other {
                None => { self.push(current); },
                Some(other) => {
                    let pair = current.subtract(other);
                    match pair.0 {
                        None => {},
                        Some(to_queue) => queue.push(to_queue)
                    }
                    match pair.1 {
                        None => {},
                        Some(to_queue) => queue.push(to_queue)
                    }
                }
            }
        }
        self
    }
}
