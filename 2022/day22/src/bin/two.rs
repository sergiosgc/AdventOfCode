use std::io::BufRead;

use aoc::{grove::Grove, move_order::MoveOrder};
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut grove_input: Vec<String> = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    let moves = Regex::new(r#"(\d+|L|R)"#)
        .unwrap()
        .captures_iter(
            &grove_input.split_off(grove_input.iter().position(|line| line.is_empty()).unwrap())[1].to_string()
        ).map(|c| c.get(1).unwrap().as_str().parse().unwrap())
        .collect::<Vec<MoveOrder>>();
    let mut grove = Grove::from_strings(grove_input);
    grove.insert_cubic_adjacencies();
    println!("{:#?}", grove.do_moves(moves));
    Ok(())
}
