use std::io::BufRead;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    println!("{:#?}", input);
    Ok(())
}