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
                (Coord{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, if newch == '*' { Ok(-1) } else { newch.to_string().parse::<i64>() })
            })
            .collect::<Vec<(Coord, Result<i64, ParseIntError>)>>()
        )
        .fold(HashMap::new(), |mut hm, item| {
            hm.insert(item.0, item.1);
            hm
        });
    println!("{:?}", schematic
        .iter()
        .map(|(coord, val)| {
            if val.is_err() || val.clone().unwrap() != -1 {
                0
            } else {
                let neighbours = coord
                    .neighbours()
                    .iter()
                    .filter(|neigh| match schematic.get(&neigh) {
                        Some(v) => match v { Ok(v) => v, Err(_) => &-1 },
                        None => &-1
                    } >= &0 )
                    .map(|c| {
                        let mut result = c.clone();
                        let left = Coord{ x: -1, y: 0};
                        while schematic.contains_key(&(result + left)) && schematic.get(&(result + left)).unwrap().is_ok() && schematic.get(&(result + left)).unwrap().clone().unwrap() != -1 { result = result + left }
                        result
                    })
                    .fold(HashMap::<Coord, bool>::new(), |mut hm, item| {
                        hm.insert(item, true);
                        hm
                    })
                    .into_keys()
                    .collect::<Vec<Coord>>();
                if neighbours.len() != 2 {
                    0
                } else {
                    neighbours.iter()
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
                        .fold(1, |acc, v| acc * v)
                }
            }
        })
        .sum::<i64>()
    );
    Ok(())
}