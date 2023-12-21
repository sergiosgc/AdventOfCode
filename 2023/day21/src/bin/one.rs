use std::{io::BufRead, collections::HashSet};
use aoc::coord::Coord;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Garden {
    pub rocks: HashSet<Coord>,
    pub dimension: Coord,
}

pub fn reachable(garden: &Garden, from: &HashSet<Coord>, steps: i64) -> HashSet<Coord> {
    if steps == 0 {
        from.clone()
    } else {
        reachable(
            garden,
            &from.iter()
                .flat_map(Coord::neighbours)
                .filter(|c| garden.rocks.get(c).is_none())
                .collect()
            ,
            steps - 1
        )
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut start_pos = Coord { x: 0, y: 0 };
    let mut dimension = Coord { x: 0, y: 0 };
    let input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .enumerate()
    .flat_map(|(y, line)| line.chars().enumerate()
        .map(|(x, ch)| { 
            if ch == 'S' { start_pos = Coord{ x: x.try_into().unwrap(), y: y.try_into().unwrap(), }; }
            dimension = Coord::bounding_max(&dimension, &Coord{ x: x.try_into().unwrap(), y: y.try_into().unwrap(), });
            (x, ch) 
        })
        .filter(|(_, ch)| *ch == '#')
        .map(|(x, _)| Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap() })
        .collect::<Vec<Coord>>()
    )
    .collect::<HashSet<Coord>>();
    println!("{:#?}", reachable(&Garden { rocks: input, dimension }, &HashSet::from([start_pos]), 64).iter().len());
    Ok(())
}
