use std::io::BufRead;
use aoc::{regex::ExtractTuple, sensor::Sensor};

use ::regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin())
        .lines().filter_map(std::io::Result::ok)
        .map( |line| Regex::new(r#"Sensor at x=([-+]?\d+), y=([-+]?\d+): closest beacon is at x=([-+]?\d+), y=([-+]?\d+)"#).unwrap().captures(&line).unwrap().extract_tuple() )
        .map( Sensor::from_tuple )
        .collect::<Vec<Sensor>>();
    
    let result = input
        .iter()
        .flat_map(|sensor| sensor.circumference())
        .filter(|coord| coord.x >= 0 && coord.x <= 4000000 && coord.y >= 0 && coord.y <= 4000000)
        .find(|coord| !input.iter().any(|sensor| sensor.covers(coord)) )
        .unwrap();
    
    println!("{}", result.x * 4000000 + result.y);

    Ok(())
}
