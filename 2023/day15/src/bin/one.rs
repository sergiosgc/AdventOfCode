use std::io::BufRead;
pub fn hash(s: &str) -> i64 {
    s.chars().fold(0_i64, |acc, ch| (ch as i64 + acc) * 17 % 256 )
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:#?}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .next().unwrap()
        .split(",")
        .map(hash)
        .sum::<i64>());
    Ok(())
}
