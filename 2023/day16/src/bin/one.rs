use std::{io::BufRead, collections::{HashMap, HashSet}};

use aoc::coord::Coord;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Mirror {
    #[default]
    None,
    RotateLeft,
    RotateRight,
    SplitVertical,
    SplitHorizontal
}
impl Mirror {
    pub fn reflect(self, vector: Coord) -> (Option<Coord>, Option<Coord>) {
        match (vector.x, self) {
            (_, Mirror::None) => (Some(vector), None),
            (0, Mirror::RotateRight) => (Some(Coord { x: vector.y, y: -vector.x }), None),
            (_, Mirror::RotateRight) => (Some(Coord { x: -vector.y, y: vector.x }), None),
            (0, Mirror::RotateLeft) => (Some(Coord { x: -vector.y, y: vector.x }), None),
            (_, Mirror::RotateLeft) => (Some(Coord { x: vector.y, y: -vector.x }), None),
            (0, Mirror::SplitVertical) => (Some(vector), None),
            (_, Mirror::SplitVertical) => (Some(Coord { x: 0, y: -1 }), Some(Coord { x: 0, y: 1}) ),
            (0, Mirror::SplitHorizontal) => (Some(Coord { x: 1, y: 0 }), Some(Coord { x: -1, y: 0}) ),
            (_, Mirror::SplitHorizontal) => (Some(vector), None),
        }
    }

}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Contraption {
    pub size: Coord,
    pub mirrors: HashMap<Coord, Mirror>,
}
pub fn energized(pos: Coord, dir: Coord, contraption: &Contraption, visited: &mut HashSet<(Coord, Coord)>) -> Vec<Coord> {
    let default_mirror = &Mirror::default();
    if !pos.inside(&Coord { x: 0, y: 0 }, &contraption.size) || visited.contains(&(pos, dir)) {
        visited.iter().map(|(coord, _)| *coord).collect()
    } else {
        visited.insert((pos, dir));
        match contraption.mirrors.get(&pos).unwrap_or(default_mirror).reflect(dir) {
            (Some(a), None) => energized(pos + a, a, contraption, visited),
            (Some(a), Some(b)) => {
                let mut with_duplicates = energized(pos + a, a, contraption, visited);
                with_duplicates.append( &mut energized(pos + b, b, contraption, visited));
                with_duplicates.into_iter()
                .collect::<HashSet<Coord>>().into_iter()
                .collect::<Vec<Coord>>()
            },
            _ => panic!("Unexpected reflect")
        }
    }


}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = {
        let input = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .enumerate().flat_map(|(y, line)| line.chars().enumerate().map(|(x, ch)| (Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap() }, match ch {
            '\\' => Mirror::RotateRight,
            '/' => Mirror::RotateLeft,
            '|' => Mirror::SplitVertical,
            '-' => Mirror::SplitHorizontal,
            _ => Mirror::None,
        }))
        .collect::<Vec<(Coord, Mirror)>>()
        )
        .collect::<Vec<(Coord, Mirror)>>();
        Contraption {
            size: input.iter().fold(Coord { x: 0, y: 0 }, |acc, (coord, _)| Coord::bounding_max(&acc, &coord)),
            mirrors: input.into_iter().filter(|(_, mirror)| *mirror != Mirror::None). collect()
        }
    };
    println!("{:#?}", energized(Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }, &input, &mut HashSet::new()).len());
    Ok(())
}
