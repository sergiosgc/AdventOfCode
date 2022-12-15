use crate::{coord::Coord, segment::Segment};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug )]
pub struct Sensor {
    pub pos: Coord,
    pub closest_beacon: Coord,
}
impl Sensor {
    pub fn from_tuple(t: (i64, i64, i64, i64)) -> Sensor {
        Sensor{
            pos: Coord { x: t.0, y: t.1 },
            closest_beacon: Coord { x: t.2, y: t.3 }
        }
    }
    pub fn intersect(self, y: i64) -> Option<Segment> {
        if (self.pos.y - y).abs() > self.manhattan_distance() {
            None
        } else {
            let remainder = self.manhattan_distance() - (self.pos.y - y).abs();
            Some(Segment::new(
                Coord::new(self.pos.x - remainder, y),
                Coord::new(self.pos.x + remainder, y)
            ))
        }
    }
    pub fn manhattan_distance(self) -> i64 { (self.pos.x - self.closest_beacon.x).abs() + (self.pos.y - self.closest_beacon.y).abs() }
    pub fn covers(self, coord: &Coord) -> bool {
        (coord.x - self.pos.x).abs() + (coord.y - self.pos.y).abs() <= self.manhattan_distance()
    }
    pub fn circumference(self) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();
        let manhattan_distance = self.manhattan_distance();
        for y in (self.pos.y - manhattan_distance - 1)..(self.pos.y + manhattan_distance + 2) {
            let remainder = manhattan_distance - (self.pos.y - y).abs() + 1;
            result.push(Coord { x: self.pos.x - remainder, y });
            if remainder > 0 {
                result.push(Coord { x: self.pos.x + remainder, y })
            }
        }
        result
    }
}
