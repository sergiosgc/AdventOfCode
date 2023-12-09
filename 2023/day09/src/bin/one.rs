use std::io::BufRead;
pub fn derive(seq: &Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut iter = seq.iter();
    let mut last = *iter.next().unwrap();
    for current in iter {
        result.push(*current - last);
        last = *current;
    }
    result
}
pub fn predict_next(seq: &Vec<i64>) -> i64 {
    if seq.iter().find(|v| **v != 0).is_some() {
        let last = seq.len() - 1;
        seq[last] + predict_next(&derive(seq))
    } else {
        0
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<Vec<i64>> = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .map(|line| line
        .split_whitespace()
        .flat_map(str::parse::<i64>)
        .collect::<Vec<i64>>()
    )
    .collect();
    println!("{:#?}", input.iter().map(|seq| predict_next(seq)).sum::<i64>());
    Ok(())
}
