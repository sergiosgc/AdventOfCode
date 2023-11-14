use std::{collections::HashMap, cmp::Ordering};
use itertools::Itertools;
use crate::{coord::{Coord, LineSortable}, line::Line, move_order::MoveOrder};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct GrovePosition {
    pub wall: bool,
    pub adjacent: HashMap<Coord, GroveAdjacency>,
}
impl GrovePosition {
}
#[derive(Clone, Default, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GroveAdjacency {
    pub pos: Coord,
    pub direction: Coord,
}
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Grove {
    pub positions: HashMap<Coord, GrovePosition>,
}
impl Grove {
    pub fn from_strings(lines: Vec<String>) -> Grove {
        lines.iter().enumerate()
            .fold(Grove { positions: HashMap::new() }, |grove, (y, line)| 
                line.chars().enumerate().filter(|(_x, ch)| *ch != ' ').fold(grove, |mut grove, (x, ch)| {
                    grove.positions.insert( Coord { x: x as i64, y: y as i64 }, GrovePosition {
                        wall: match ch {
                            '.' => false,
                            '#' => true,
                            _ => panic!("Unexpected char {}", ch),
                        },
                        ..Default::default()
                    });
                    grove
                })
            )
    }
    pub fn insert_adjacencies(&mut self) {
        {
            let max_y = self.positions.keys().map(|coord| coord.y).max().unwrap();
            for y in 0..max_y+1 {
                let max_x = self.positions.iter().filter(|(coord, _position)| coord.y == y).map(|(coord, _position)| coord.x).max().unwrap();
                let min_x = self.positions.iter().filter(|(coord, _position)| coord.y == y).map(|(coord, _position)| coord.x).min().unwrap();
                self.sew_tiles(Coord{ x: min_x, y }, Coord{ x: max_x, y }, Coord { x: -1, y: 0}, Coord { x: 1, y: 0 });
            }
        }
        {
            let max_x = self.positions.keys().map(|coord| coord.x).max().unwrap();
            for x in 0..max_x+1 {
                let max_y = self.positions.iter().filter(|(coord, _position)| coord.x == x).map(|(coord, _position)| coord.y).max().unwrap();
                let min_y = self.positions.iter().filter(|(coord, _position)| coord.x == x).map(|(coord, _position)| coord.y).min().unwrap();
                self.sew_tiles(Coord{ x, y: min_y }, Coord{ x, y: max_y }, Coord { x: 0, y: -1}, Coord { x: 0, y: 1 });
            }
        }
    }
    pub fn insert_cubic_adjacencies(&mut self) {
        let mut outer_border = {
            let mut unsorted = self.positions.keys().flat_map(|coord| coord.neighbours()).filter(|coord| !self.positions.contains_key(&coord)).unique().collect::<Vec<Coord>>();
            let start = *unsorted.get(0).unwrap();
            unsorted.sort_into_line(start)
        };
        
        let inner_corners = outer_border.iter().filter(|candidate| candidate.neighbours().into_iter().filter(|n| self.positions.contains_key(n)).count() == 5).copied().collect::<Vec<Coord>>();
        let outer_corners = outer_border.iter().filter(|candidate| candidate.neighbours().into_iter().filter(|n| self.positions.contains_key(n)).count() == 1).copied().collect::<Vec<Coord>>();
        outer_border.retain(|c| !(inner_corners.contains(c) || outer_corners.contains(c)));
        for inner_corner in inner_corners.clone() {
            let mut target_outer_corners = outer_corners.iter().filter(|candidate| candidate.x == inner_corner.x || candidate.y == inner_corner.y)
                .filter(|outer_corner| {
                    let line = Line { from: inner_corner, to: **outer_corner };
                    inner_corner.neighbours().iter().filter(|n| !self.positions.contains_key(n)).filter(|n| line.contains(n)).count() == 1
                })
                .copied()
                .map(|c| vec![c])
                .collect::<Vec<Vec<Coord>>>();
            if target_outer_corners.len() != 2 { panic!("Unexpected"); }
            if inner_corner.levenshtein_distance(target_outer_corners.get(0).unwrap().get(0).unwrap()) < inner_corner.levenshtein_distance(target_outer_corners.get(1).unwrap().get(0).unwrap()) {
                let corner = target_outer_corners.pop().unwrap();
                target_outer_corners.insert(0, corner);
            }
            if inner_corner.levenshtein_distance(target_outer_corners.get(0).unwrap().get(0).unwrap()) > inner_corner.levenshtein_distance(target_outer_corners.get(1).unwrap().get(0).unwrap()) {
                let corner = *target_outer_corners.get(1).unwrap().get(0).unwrap();
                target_outer_corners.get_mut(1).unwrap().push(*outer_corners
                    .iter()
                    .filter(|c| **c != corner && (c.x == corner.x || c.y == corner.y))
                    .reduce(|best, c| if corner.levenshtein_distance(c) < corner.levenshtein_distance(best) { c } else { best })
                    .unwrap()
                    );
            }
            let coords_to_sew = target_outer_corners.into_iter().map(|target_outer_corner| [vec![inner_corner], target_outer_corner]
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<Coord>>()
                .windows(2)
                .flat_map(|slice| (Line { from: slice[0], to: slice[1] }).coords() )
                .filter(|c| !outer_corners.contains(c))
                .collect::<Vec<Coord>>())
                .collect::<Vec<Vec<Coord>>>();
            coords_to_sew[0].iter().zip(coords_to_sew[1].iter()).for_each(|(a, b)| self.sew_cubic_tiles(*a, *b));
        }
        let mut stragglers_a = outer_corners.clone().iter()
            .filter(|c| c.neighbours().iter().filter(|n| self.positions.contains_key(n)).count() == 1)
            .map(|c| *c.neighbours().iter().find(|n| self.positions.contains_key(n)).unwrap() )
            .fold(
                outer_border.iter()
                        .flat_map(|b| b.neighbours())
                        .filter(|c| self.positions.contains_key(c))
                        .unique()
                        .filter(|c| self.positions.get(c).unwrap().adjacent.is_empty())
                        .fold(Vec::<Coord>::new(), |mut acc, c| {
                            if acc.is_empty() || acc.iter().any(|coord| coord.x == c.x || coord.y == c.y) {
                                acc.push(c);
                            }
                            acc
                        }),
                |mut acc, corner| {
                    if acc.iter().any(|c| corner.levenshtein_distance(c) == 1) {
                        acc.push(corner);
                    }
                    acc
                });
        stragglers_a.sort();
        let mut stragglers_b = outer_corners.clone().iter()
            .filter(|c| c.neighbours().iter().filter(|n| self.positions.contains_key(n)).count() == 1)
            .map(|c| *c.neighbours().iter().find(|n| self.positions.contains_key(n)).unwrap() )
            .fold(
            outer_border.iter()
                    .flat_map(|b| b.neighbours())
                    .filter(|c| self.positions.contains_key(c))
                    .unique()
                    .filter(|c| self.positions.get(c).unwrap().adjacent.is_empty())
                    .filter(|c| !stragglers_a.contains(c))
                    .collect::<Vec<Coord>>(),
                |mut acc, corner| {
                    if acc.iter().any(|c| corner.levenshtein_distance(c) == 1) {
                        acc.push(corner);
                    }
                    acc
                });
        stragglers_b.sort();
        stragglers_b.reverse();
        stragglers_a.iter().zip(stragglers_b.iter()).for_each(|(a, b)| self.sew_cubic_tiles_from_inner(*a, *b));
    }
    pub fn sew_cubic_tiles_from_inner(&mut self, inner_a: Coord, inner_b: Coord) {
        let outer_a = *inner_a
            .neighbours()
            .iter()
            .filter(|c| !self.positions.contains_key(c))
            .find(|c| !self.positions.get(&inner_a).unwrap().adjacent.contains_key(c))
            .unwrap();
        let outer_b = *inner_b
            .neighbours()
            .iter()
            .filter(|c| !self.positions.contains_key(c))
            .find(|c| !self.positions.get(&inner_a).unwrap().adjacent.contains_key(c))
            .unwrap();



        self.sew_tiles(inner_a, inner_b, outer_a - inner_a, outer_b - inner_b);
    }
    pub fn sew_cubic_tiles(&mut self, outer_a: Coord, outer_b: Coord) {
        let (inner_a, inner_b) = if outer_a == outer_b {
            outer_a.neighbours()[0..4].iter().filter(|c| self.positions.contains_key(c)).copied().next_tuple().unwrap()
        } else {
            (
                *outer_a.neighbours()[0..4].iter().find(|c| self.positions.contains_key(c)).unwrap(),
                *outer_b.neighbours()[0..4].iter().find(|c| self.positions.contains_key(c)).unwrap()
            )
        };
        if inner_b == (Coord { x: 14, y: 10 }) {
            panic!("Here {:?}", outer_b);
        }
        self.sew_tiles(inner_a, inner_b, outer_a - inner_a, outer_b - inner_b);
    }
    pub fn sew_tiles(&mut self, a: Coord, b: Coord, exit_direction_a: Coord, exit_direction_b: Coord) {
        let option_position_a = self.positions.get(&a);
        let option_position_b = self.positions.get(&b);
        if option_position_a.is_none() || option_position_b.is_none() { return; }
        self.positions.get_mut(&a).unwrap().adjacent.insert(a + exit_direction_a, GroveAdjacency { pos: b, direction: -exit_direction_b });
        self.positions.get_mut(&b).unwrap().adjacent.insert(b + exit_direction_b, GroveAdjacency { pos: a, direction: -exit_direction_a });
    }
    pub fn do_moves(self, moves: Vec<MoveOrder>) -> i64 {
        let mut current_pos = *self.positions.iter().reduce(|best, current| 
            if best.1.wall || !current.1.wall && (current.0.y < best.0.y || current.0.y == best.0.y && current.0.x < best.0.x) {
                current
            } else { best }
        ).unwrap().0;
        let mut current_direction = Coord{ x: 1, y: 0 };
        for move_ in moves {
            match move_ {
                MoveOrder::Forward(count) => {
                    for _i in 0..count {
                        let non_wrap_next_position = current_pos + current_direction;
                        let (candidate_position, candidate_direction) = match self.positions.get(&current_pos).unwrap().adjacent.get(&non_wrap_next_position) {
                            Some(adjacency) => (adjacency.pos, adjacency.direction),
                            None => (non_wrap_next_position, current_direction)
                        };
                        if !self.positions.get(&candidate_position).unwrap().wall { 
                            current_pos = candidate_position;
                            current_direction = candidate_direction;
                        }
                    }
                },
                MoveOrder::TurnLeft => { current_direction = Coord { x: current_direction.y, y: -current_direction.x } },
                MoveOrder::TurnRight => { current_direction = Coord { x: -current_direction.y, y: current_direction.x } },
            }
        }

        1000_i64 * (current_pos.y+1) + 4_i64 * (current_pos.x+1) + match current_direction {
            Coord { x: 1, y: 0 } => 0,
            Coord { x: 0, y: 1 } => 1,
            Coord { x: -1, y: 0 } => 2,
            Coord { x: 0, y: -1 } => 3,
            _ => panic!("Unexpected")
        }
    }

}
