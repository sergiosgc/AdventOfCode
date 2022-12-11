use std::io::BufRead;
use itertools::Itertools;
use aoc::monkey::{Monkey, MonkeyExecutor};
use num::integer::lcm;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut monkeys = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .collect::<Vec<String>>()
        .chunks(7)
        .map(|m| Monkey::from_string(m))
        .collect::<Vec<Monkey>>();
    let lcm = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .reduce(|a,b| lcm(a, b))
        .unwrap();
    for _i in 0..20 {
        monkeys = monkeys.execute_round(3, lcm);
    }
    println!("{:?}", monkeys
        .iter()
        .map(|m| m.throw_count)
        .sorted_by(|a,b| Ord::cmp(b,a) )
        .take(2)
        .reduce(|acc,v| acc * v)
        .unwrap());
    Ok(())
}
