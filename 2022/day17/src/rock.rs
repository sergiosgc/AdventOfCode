use crate::coord::Coord;

#[derive(Clone, Debug)]
pub struct Rock {
   pub occupied: Vec<Coord>,
}
impl Rock {
    pub fn new(type_: i64) -> Rock {
        match type_ {
            0 => Rock {
                occupied: vec![
                    Coord { x: 0, y: 0 },
                    Coord { x: 1, y: 0 },
                    Coord { x: 2, y: 0 },
                    Coord { x: 3, y: 0 },
                ]
            },
            1 => Rock {
                occupied: vec![
                    Coord { x: 1, y: 1 },
                    Coord { x: 0, y: 1 },
                    Coord { x: 2, y: 1 },
                    Coord { x: 1, y: 0 },
                    Coord { x: 1, y: 2 },
                ]
            },
            2 => Rock {
                occupied: vec![
                    Coord { x: 0, y: 0 },
                    Coord { x: 1, y: 0 },
                    Coord { x: 2, y: 0 },
                    Coord { x: 2, y: 1 },
                    Coord { x: 2, y: 2 },
                ]
            },
            3 => Rock {
                occupied: vec![
                    Coord { x: 0, y: 3 },
                    Coord { x: 0, y: 2 },
                    Coord { x: 0, y: 1 },
                    Coord { x: 0, y: 0 },
                ]
            },
            4 => Rock {
                occupied: vec![
                    Coord { x: 0, y: 0 },
                    Coord { x: 1, y: 0 },
                    Coord { x: 1, y: 1 },
                    Coord { x: 0, y: 1 },
                ]
            },
            _ => panic!("Unknown rock type {}", type_)
        }
    }
}