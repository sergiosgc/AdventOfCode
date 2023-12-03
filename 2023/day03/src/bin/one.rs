use std::{io::BufRead, num::ParseIntError, collections::HashMap};
use aoc::coord::Coord;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schematic: HashMap<Coord, Result<i64, ParseIntError>> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter(|(_x, ch)| *ch != '.')
            .map(move |(x, ch)| {
                let newch = ch.clone();
                (Coord{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, newch.to_string().parse::<i64>())
            })
            .collect::<Vec<(Coord, Result<i64, ParseIntError>)>>()
        )
        .fold(HashMap::new(), |mut hm, item| {
            hm.insert(item.0, item.1);
            hm
        });
    println!("{:?}", schematic
        .iter()
        .flat_map(|(coord, val)| 
            if val.is_ok() {
                vec![]
            } else {
                coord.neighbours()
                    .iter()
                    .filter(|coord| schematic.contains_key(coord) && schematic.get(coord).unwrap().is_ok())
                    .map(|coord| coord.clone())
                    .collect::<Vec<Coord>>()
            }
        )
        .map(|coord| {
            let mut result = coord.clone();
            let left = Coord{ x: -1, y: 0};
            while schematic.contains_key(&(result + left)) && schematic.get(&(result + left)).unwrap().is_ok() { result = result + left }
            result
        })
        .fold(HashMap::<Coord, bool>::new(), |mut hm, item| {
            hm.insert(item, true);
            hm
        })
        .into_keys()
        .map(|coord| {
            let mut current_coord = coord.clone();
            let mut result: i64 = schematic.get(&coord).unwrap().clone().unwrap();
            let right = Coord{ x: 1, y: 0};
            while schematic.contains_key(&(current_coord + right)) && schematic.get(&(current_coord + right)).unwrap().is_ok() { 
                result = result * 10 + schematic.get(&(current_coord + right)).unwrap().clone().unwrap();
                current_coord = current_coord + right;
            }
            result
        })
        .sum::<i64>()
//        .collect::<Vec<i64>>()
    );
    Ok(())
}
