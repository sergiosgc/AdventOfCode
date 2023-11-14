use crate::coord::{Coord, CardinalPoint};

pub static MOVE_PROPOSALS: [ElfMoveProposal; 4] = [
    ElfMoveProposal { 
        vector: Coord::from_cardinal(CardinalPoint::N),
        must_be_empty: [
            Coord::from_cardinal(CardinalPoint::N),
            Coord::from_cardinal(CardinalPoint::NE),
            Coord::from_cardinal(CardinalPoint::NW),
        ]
    },
    ElfMoveProposal { 
        vector: Coord::from_cardinal(CardinalPoint::S),
        must_be_empty: [
            Coord::from_cardinal(CardinalPoint::S),
            Coord::from_cardinal(CardinalPoint::SE),
            Coord::from_cardinal(CardinalPoint::SW),
        ]
    },
    ElfMoveProposal { 
        vector: Coord::from_cardinal(CardinalPoint::W),
        must_be_empty: [
            Coord::from_cardinal(CardinalPoint::W),
            Coord::from_cardinal(CardinalPoint::NW),
            Coord::from_cardinal(CardinalPoint::SW),
        ]
    },
    ElfMoveProposal { 
        vector: Coord::from_cardinal(CardinalPoint::E),
        must_be_empty: [
            Coord::from_cardinal(CardinalPoint::E),
            Coord::from_cardinal(CardinalPoint::NE),
            Coord::from_cardinal(CardinalPoint::SE),
        ]
    },
];
pub struct ElfMoveProposal {
    vector: Coord,
    must_be_empty: [Coord; 3],
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Elf {
}
impl Elf {
    pub fn plan_move(self, round: i64, current: &Coord, grove: &[Coord]) -> Option<Coord> {
        if current.neighbours().iter().any(|neighbour| grove.contains(neighbour)) {
            for proposal_index in 0..4 {
                if !MOVE_PROPOSALS[((round + proposal_index) % 4) as usize]
                    .must_be_empty
                    .into_iter()
                    .map(|must_be_empty| *current + must_be_empty)
                    .any(|target_coord| grove.contains(&target_coord)) {
                        return Some(*current + MOVE_PROPOSALS[((round + proposal_index) % 4) as usize].vector);
                }
            }
            None
        } else {
            None
        }
    }
}