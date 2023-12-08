use std::{io::BufRead, collections::HashMap};
pub fn solve_for(start: String, network: &HashMap<String, (String, String)>, instructions: &Vec<char>) -> i64 {
    let mut instructions_iter = instructions.into_iter().cycle().enumerate();
    let mut current = start.clone();
    while current.clone().split_off(2) != "Z".to_string() {
        current = match instructions_iter.next().unwrap().1 {
            'L' => network.get(&current).unwrap().0.clone(),
            _ => network.get(&current).unwrap().1.clone() 
        };
    }
    instructions_iter.next().unwrap().0 as i64
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok);
    let instructions = input.next().unwrap().chars().collect::<Vec<char>>();
    _ = input.next();
    let network = input
    .fold(HashMap::<String, (String, String)>::new(), |mut acc, line| {
        let (key, rest) = line
        .split_once("=")
        .unwrap();
        let (left, right) = rest.split_once(",").unwrap();
        acc.insert(
            key.replace(" ", "").to_string(), 
            (left.replace(" ", "").replace("(", "").to_string(),
               right.replace(" ", "").replace(")", "").to_string())
        );
        acc
    });

    println!("{:#?}", network
    .clone()
    .into_iter()
    .map(|v| v.0)
    .filter(|node| node.clone().split_off(2) == "A".to_string())
    .map(|start| solve_for(start, &network, &instructions))
    .fold(1_i64, num::integer::lcm));
    Ok(())
}