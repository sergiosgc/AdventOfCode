use std::io::BufRead;
use aoc::{coord::Coord, rope::Rope};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok);
    println!("{:#?}", input
        .map(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            (match split[0] {
                "R" => Coord::new(1, 0),
                "L" => Coord::new(-1, 0),
                "U" => Coord::new(0, 1),
                "D" => Coord::new(0, -1),
                _ => panic!("Unexpected direction")
            }) * split[1].parse::<i64>().unwrap()
        })
        .fold(Rope::new(2), |rope, head_move| rope.move_head(head_move) )
        .tail_positions.len()
    );
    Ok(())
}
