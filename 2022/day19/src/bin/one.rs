use std::io::BufRead;
use aoc::{regex::ExtractTuple7, recipe::Recipe};
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<Recipe> = std::io::BufReader::new(std::io::stdin())
    .lines().filter_map(std::io::Result::ok)
    .map(|line| Regex::new(r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap()
        .captures(&line).unwrap()
        .extract_tuple()
    )
    .map(Recipe::from_tuple)
    .collect();
    println!("{:#?}", input.into_iter().map(|recipe| recipe.quality(24)).sum::<i64>());
    Ok(())
}
