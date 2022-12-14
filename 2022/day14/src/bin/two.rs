use std::io::BufRead;
use itertools::Itertools;

use aoc::{coord::Coord, segment::Segment, cave::Cave};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Cave{ sands: Vec::new(), segments: std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            line.split(" -> ")
                .map(|coord| Coord::from_string(coord))
                .tuple_windows::<(Coord, Coord)>()
                .map(|(a, b)| Segment::new(a,b) )
                .collect::<Vec<Segment>>()
        })
        .flatten()
        .collect::<Vec<Segment>>()};
    let void_y = input.segments.iter().map(|segment| segment.end.y).max().unwrap() + 2;
    input.segments.push( Segment::new(
        Coord::new(std::i64::MIN, void_y),
        Coord::new(std::i64::MAX, void_y),
    ));
    input.fill_sand(Coord::new(500, 0));
    println!("{:#?}", input.sands.len());
    Ok(())
}
