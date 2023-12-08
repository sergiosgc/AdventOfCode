use std::{io::BufRead, collections::HashMap};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok);
    let mut instructions = input.next().unwrap().chars().collect::<Vec<char>>().into_iter().cycle().enumerate();
    _ = input.next();
    let network = input
    .fold(HashMap::<String, (String, String)>::new(), |mut acc, line| {
        let (key, rest) = line
        .split_once("=")
        .unwrap();
        let (left, right) = rest.split_once(",").unwrap();
        acc.insert(key.replace(" ", "").to_string(), (left.replace(" ", "").replace("(", "").to_string(), right.replace(" ", "").replace(")", "").to_string()));
        acc
    });
    let mut current = "AAA".to_string();
    let count = {
        while current != "ZZZ".to_string() {
            current = match instructions.next().unwrap().1 {
                'L' => network.get(&current).unwrap().0.clone(),
                _ => network.get(&current).unwrap().1.clone() 
            };
        }
        instructions.next().unwrap().0
    };

    println!("{:#?}", count);
    Ok(())
}
