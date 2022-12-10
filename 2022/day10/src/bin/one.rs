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
        .map(|(cycle, cpu)| (cycle+1, cpu))
        .filter(|(cycle, _cpu)| cycle % 40 == 20)
        .map(|(cycle, cpu)| cycle as i64 * cpu.x)
        .sum::<i64>()
    );
    Ok(())
}
