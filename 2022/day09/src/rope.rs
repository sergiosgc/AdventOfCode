use std::collections::HashSet;
use crate::coord::Coord;
#[derive(Clone, Debug)]
pub struct Rope {
    pub knots: Vec<Coord>,
    pub tail_positions: HashSet<Coord>
}
impl Rope {
    pub fn new(knot_count: i64) -> Rope {
        Rope { 
            knots: {
                let mut knots = Vec::<Coord>::new();
                for _i in 0..knot_count { knots.push(Coord::new(0,0)); }
                knots
            },
            tail_positions: HashSet::from_iter([Coord::new(0,0)]) } 
    }
    pub fn move_head(mut self, vector: Coord) -> Rope {
        if vector.length() == 1.0 {
            let mut knots=vec![ self.knots[0] + vector ];
            knots.append(&mut Rope::move_knots(self.knots[1..].to_vec())); 
            self.knots = Rope::move_knots(knots);
            self.tail_positions.insert(self.knots[self.knots.len() - 1]);
            self
        } else {
            self.move_head(vector.unit()).move_head(vector - vector.unit())
        }
    }
    pub fn move_knots(mut knots: Vec<Coord>) -> Vec<Coord> {
        if knots.len() == 1 { 
            knots 
        } else {
            let diff = knots[0] - knots[1]; 
            if diff.length() > 2_f64.sqrt() {
                knots[1] = knots[1] + Coord::new( 
                    if diff.x == 0 { 0 } else { diff.x / diff.x.abs() },
                    if diff.y == 0 { 0 } else { diff.y / diff.y.abs() }
                );
            }
            [ vec![ knots[0] ], Rope::move_knots(knots[1..].to_vec()) ].into_iter().flatten().collect::<Vec<Coord>>()
        }
    }
}