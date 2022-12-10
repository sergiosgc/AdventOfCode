use std::io::BufRead;

use aoc::{opcode::Opcode, cpu::CycleIterator};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok);
    println!("{:#?}", CycleIterator::new(
            input
                .map(|line| Opcode::from_str(&line))
                .collect::<Vec<Opcode>>()
        )
        .enumerate()
        .map(|(cycle, cpu)| (cycle as i64 % 40, cpu.x))
        .map(|(screenx, spritex)| if (screenx - spritex).abs() <= 1 { '#' } else { '.' } )
        .collect::<Vec<char>>()
        .chunks_exact(40)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
    );
    Ok(())
}
