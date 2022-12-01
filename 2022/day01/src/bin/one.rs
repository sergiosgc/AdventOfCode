use std::io::BufRead;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:#?}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .fold((0,0), |(max, current), line| {
            if line.len() == 0 {
                (max.max(current), 0)
            } else {
                (max, current + line.parse::<i64>().unwrap())
            }
        }).0
    );
    Ok(())
}
