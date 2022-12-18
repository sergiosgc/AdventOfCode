use std::{fmt, iter};

use crate::{coord::Coord, rock::Rock};

#[derive(Clone)]
pub struct Cave {
   pub occupied: Vec<Coord>,
   pub jet_pattern: Vec::<Coord>,
   pub jet_pattern_iterations: usize,
}
impl Cave {
    pub fn new(jet_pattern: Vec<Coord>) -> Cave { Cave { occupied: Vec::new(), jet_pattern, jet_pattern_iterations: 0 } }
    pub fn tallest_y(&self) -> i64 {
        self.occupied
            .iter()
            .map(|coord| coord.y)
            .max()
            .unwrap_or(0)
    }
    pub fn drop_rock(&mut self, rock: Rock) {
        let mut current_pos = Coord { x: 3, y: self.tallest_y() + 4 };
        let jet_pattern_iterator = iter::repeat(self.jet_pattern.clone().into_iter()).flatten().skip(self.jet_pattern_iterations);
        for jet in jet_pattern_iterator {
            self.jet_pattern_iterations = (self.jet_pattern_iterations + 1) % self.jet_pattern.len();
            let after_jet_pos = current_pos + jet;
            if self.fits(&rock, after_jet_pos) {
                current_pos = after_jet_pos;
            }
            let after_drop_pos = current_pos + Coord { x: 0, y: -1 };
            if self.fits(&rock, after_drop_pos) {
                current_pos = after_drop_pos;
            } else {
                self.insert_rock(rock, current_pos);
                return;
            }
        }
        panic!("Ran out of jets");
    }
    pub fn fits(&self, rock: &Rock, pos: Coord) -> bool {
        !rock.occupied.clone().into_iter().any(|rock_coord| !self.is_free(rock_coord + pos) )
    }
    pub fn is_free(&self, coord: Coord) -> bool {
        coord.x > 0 && coord.x < 8 && coord.y > 0 && !self.occupied.contains(&coord)
    }
    pub fn insert_rock(&mut self, rock: Rock, pos: Coord) {
        self.occupied.append(&mut rock.occupied.into_iter().map(|c| c + pos).collect::<Vec<Coord>>());
    }
}
impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.tallest_y()+4).rev() {
            for x in 0_i64..9 {
                write!(f, "{}", if self.occupied.contains(&Coord{ x, y }) { "#" } else { 
                    let chars = if y == 0 { vec!["+", "-"] } else { vec!["|", "."] };
                    if x == 0 || x == 8 { chars[0] } else { chars[1] }
                })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
