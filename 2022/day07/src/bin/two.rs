use std::{io::BufRead};
use aoc::shell::Shell;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut shell: Shell = Shell::new();
    for line in std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok) {
        shell = shell.consume_input(line);
    }
    let min_to_free = 30000000 - (70000000 - shell.root.clone().size());
    println!("{}", shell.root.sizes()
        .into_iter()
        .filter(|size| *size >= min_to_free)
        .reduce(|a,b| a.min(b))
        .unwrap());
    Ok(())
}
