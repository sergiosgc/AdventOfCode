use std::io::BufRead;
use aoc::{regex::ExtractTuple4, sensor::Sensor};

use ::regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<Sensor> = std::io::BufReader::new(std::io::stdin())
        .lines().filter_map(std::io::Result::ok)
        .map( |line| ExtractTuple4::<i64, i64, i64, i64>::extract_tuple(Regex::new(r#"Sensor at x=([-+]?\d+), y=([-+]?\d+): closest beacon is at x=([-+]?\d+), y=([-+]?\d+)"#).unwrap().captures(&line).unwrap()) )
        .map( Sensor::from_tuple )
        .collect();
    input.reverse();
    let result = input
        .iter()
        .flat_map(|sensor| sensor.circumference())
        .filter(|coord| coord.x >= 0 && coord.x <= 4000000 && coord.y >= 0 && coord.y <= 4000000)
        .find(|coord| !input.iter().any(|sensor| sensor.covers(coord)) )
        .unwrap();
    
    println!("{}", result.x * 4000000 + result.y);

    Ok(())
}
