use std::io::BufRead;
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    println!("{:#?}", input
        .into_iter()
        .map(|line| Regex::new(r"(.)(?!\1)(.)(?!\1|\2)(.)(?!\1|\2|\3)(.)(?!\1|\2|\3|\4)")
            .unwrap()
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .end()
        )
        .collect::<Vec<usize>>()
    );
    Ok(())
}
