use std::io::BufRead;

use aoc::packet::Packet;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok);
    println!("{:#?}", input
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|pair| (Packet::from_string(pair[0].clone()), Packet::from_string(pair[1].clone())) )
        .map(|(left, right)| left < right)
        .enumerate()
        .filter(|(_index, right_order)| *right_order)
        .map(|(index, _right_order)| index + 1)
        .sum::<usize>()

    );
    Ok(())
}
