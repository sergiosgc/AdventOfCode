use std::io::BufRead;

use aoc::calculator::Calculator;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    let calculator = Calculator::from_strings(input);
    println!("{:#?}", calculator.calculate(&"root".to_string(), None));
    Ok(())
}
