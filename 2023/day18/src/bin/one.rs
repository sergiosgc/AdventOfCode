use std::{io::BufRead, collections::HashSet};

use aoc::coord::Coord;
pub fn print_map(map: &HashSet<Coord>) {
    let dimensions = Coord {
        x: map.iter().map(|c| c.x).max().unwrap(),
        y: map.iter().map(|c| c.y).max().unwrap(),
    };
    for y in 0..dimensions.y + 1 {
        for x in 0..dimensions.x+1 {
            print!("{}", if map.contains(&Coord { x, y }) { "#" } else { "." });
        }
        println!("");
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut digged_coords = vec![Coord::new(0,0)];
    let mut input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .map(|line| {
        let mut iter = line.split(" ");
        let direction = match iter.next().unwrap() {
            "U" => Coord::new(0,-1),
            "D" => Coord::new(0,1),
            "L" => Coord::new(-1,0),
            _ => Coord::new(1,0),
        };
        let length = iter.next().unwrap().parse::<i64>().unwrap();
        let color = iter.next().unwrap().replace(&['(', ')'], "");
        (direction, length, color)
    })
    .fold(&mut digged_coords, |acc: &mut Vec<Coord>, (direction, length, _)| {
        let mut current = acc[acc.len() -1 ];
        for _ in 0..length {
            current = current + direction;
            acc.push(current);
        }
        acc
    })
    .iter()
    .map(Coord::clone)
    .collect::<HashSet<Coord>>();
    let to_shift = Coord {
        x: -input.iter().map(|c| c.x).min().unwrap(),
        y: -input.iter().map(|c| c.y).min().unwrap(),
    };
    input = input.iter().map(|c| *c + to_shift).collect::<HashSet<Coord>>();
    let dimensions = Coord {
        x: input.iter().map(|c| c.x).max().unwrap(),
        y: input.iter().map(|c| c.y).max().unwrap(),
    };
    for y in 0..dimensions.y+1 {
        let mut inside = false;
        for x in 0..dimensions.x + 1 {
            if input.contains(&Coord { x, y }) && input.contains(&Coord { x, y: y+1 }) { inside = !inside; }
            if inside { input.insert( Coord { x, y }); }
        }
    }
    println!("{:#?}", input.iter().len() );
    Ok(())
}
