use std::{io::BufRead, iter};

use aoc::{rock::Rock, cave::Cave, coord::Coord};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jet_pattern = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .flat_map(|s| s.chars().collect::<Vec<char>>() )
        .flat_map(|c| match c {
            '<' => Some(Coord{ x: -1, y: 0} ),
            '>' => Some(Coord{ x: 1, y: 0} ),
            _ => None
        })
        .collect::<Vec<Coord>>();
    println!("{:?}", 
        iter::repeat(vec![
            Rock::new(0),
            Rock::new(1),
            Rock::new(2),
            Rock::new(3),
            Rock::new(4),
        ].into_iter())
        .flatten()
        .take(2022)
        .fold(Cave::new(jet_pattern), |mut cave, rock| { cave.drop_rock(rock); cave } )
        .tallest_y()
    );
    Ok(())
}
