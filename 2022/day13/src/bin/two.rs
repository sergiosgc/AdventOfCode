use std::io::BufRead;

use aoc::packet::Packet;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .filter(|line| line.len() > 0)
        .map(|line| Packet::from_string(line))
        .collect::<Vec<Packet>>();
    let divider_packets = vec![
        Packet::from_string("[[2]]".to_string()),
        Packet::from_string("[[6]]".to_string()),
    ];
    input.sort();
    println!("{:#?}", input
        .iter()
        .enumerate()
        .filter(|(_i, packet)| *packet == &divider_packets[0] || *packet == &divider_packets[1])
        .map(|(i, _packet)| i + 1)
        .product::<usize>()
    );
    Ok(())
}
