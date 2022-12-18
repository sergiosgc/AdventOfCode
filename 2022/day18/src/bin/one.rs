use std::io::BufRead;
use aoc::{regex::ExtractTuple3, coord::Coord};

use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<Coord> = std::io::BufReader::new(std::io::stdin())
        .lines().filter_map(std::io::Result::ok)
        .map(|line| Regex::new(r#"^(\d+),(\d+),(\d+)$"#).unwrap()
            .captures(&line).unwrap()
            .extract_tuple()
        )
        .map(|(x, y, z)| Coord { x, y, z })
        .collect();
    println!("{:#?}", input
        .iter()
        .flat_map(|cube| cube.neighbours())
        .filter(|cube| !input.contains(cube))
        .count()
    );
    Ok(())
}