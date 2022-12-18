use std::{io::BufRead, iter};
const QUIESCENCE_DELTA: usize = 1000;
use aoc::{rock::Rock, cave::Cave, coord::Coord};
pub fn is_cycle(deltas: &Vec<i64>) -> bool {
    if deltas.len() > QUIESCENCE_DELTA && (deltas.len() - QUIESCENCE_DELTA) / 2 > 200 {
        let half_len = (deltas.len() - QUIESCENCE_DELTA) / 2;
        let left = deltas[QUIESCENCE_DELTA..QUIESCENCE_DELTA + half_len+1].to_vec();
        let right = deltas[QUIESCENCE_DELTA + half_len..].to_vec();
        left.len() == right.len() && !left.iter().enumerate().any(|(i, d)| &right[i] != d)
    } else {
        false
    }
}
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
    let mut cave = Cave::new(jet_pattern);
    let mut last_height = cave.tallest_y();
    let mut height_deltas = Vec::<i64>::new();
    for rock in iter::repeat(vec![
            Rock::new(0),
            Rock::new(1),
            Rock::new(2),
            Rock::new(3),
            Rock::new(4),
        ].into_iter())
        .flatten() {
            cave.drop_rock(rock);
            height_deltas.push(cave.tallest_y() - last_height);
            last_height = cave.tallest_y();
            if is_cycle(&height_deltas) {
                let rock_count: i64 = 1000000000000;
                let cycle_length: i64 = ((height_deltas.len() - QUIESCENCE_DELTA) / 2).try_into().unwrap();
                let num_cycles: i64 = (rock_count - QUIESCENCE_DELTA as i64) / cycle_length;
                let cycle_tail: i64 = (rock_count - QUIESCENCE_DELTA as i64) % cycle_length;

                let quiescence_height: i64 = height_deltas[0..QUIESCENCE_DELTA].into_iter().sum();
                let cycle_height: i64 = height_deltas[QUIESCENCE_DELTA..QUIESCENCE_DELTA+cycle_length as usize].into_iter().sum();
                let tail_height: i64 = height_deltas[QUIESCENCE_DELTA..QUIESCENCE_DELTA+cycle_tail as usize].into_iter().sum();
                println!("num cycles:{}, cycle tail: {}, cycle_length {}, quiescence_height: {}, cycle_height: {}, tail_height: {}", num_cycles, cycle_tail, cycle_length, quiescence_height, cycle_height, tail_height);
                println!("{}", quiescence_height + tail_height + cycle_height * num_cycles);
                return Ok(())
            }
        }
    Ok(())
}
