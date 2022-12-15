use std::io::BufRead;
use aoc::{regex::ExtractTuple, sensor::Sensor, segment::{Segment, InsertNonOverlapping}};
use itertools::Itertools;

use ::regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin())
        .lines().filter_map(std::io::Result::ok)
        .map( |line| Regex::new(r#"Sensor at x=([-+]?\d+), y=([-+]?\d+): closest beacon is at x=([-+]?\d+), y=([-+]?\d+)"#).unwrap().captures(&line).unwrap().extract_tuple() )
        .map( Sensor::from_tuple )
        .collect::<Vec<Sensor>>();
    let segments = input
        .iter()
        .flat_map( |sensor| sensor.intersect(2000000) )
        .fold(Vec::<Segment>::new(), |segments, segment| segments.insert_non_overlapping(segment));
    
    println!("{:#?}", segments
        .iter()
        .map(|segment| segment.length())
        .sum::<i64>()
        - input
        .iter()
        .map( |sensor| sensor.closest_beacon )
        .filter( |beacon| segments.iter().any(|segment| segment.start.y == beacon.y && segment.start.x <= beacon.x && segment.end.x >= beacon.x ))
        .unique()
        .count() as i64
    );

    Ok(())
}
