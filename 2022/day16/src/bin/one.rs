use std::io::BufRead;
use aoc::cave::Cave;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Cave::from_string(std::io::BufReader::new(std::io::stdin())
        .lines().filter_map(std::io::Result::ok) );
    println!("{:#?}", input.recursive_pressure("AA".to_string(), 30, 0, Vec::new()));
    Ok(())
}
