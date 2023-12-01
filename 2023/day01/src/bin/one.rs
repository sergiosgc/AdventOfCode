use std::io::BufRead;
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r#"^[^0-9]*(?P<first>\d).*?(?P<last>\d)?[^0-9]*$"#).unwrap();
    println!("{}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            let captures = re.captures(&line).unwrap();
            match (captures.name("first"), captures.name("last")) {
                (None, None) => panic!("No digits matched"),
                (None, Some(_)) => panic!("Impossible match"),
                (Some(first), None) => format!("{}{}", first.as_str(), first.as_str()).parse::<i32>().unwrap(),
                (Some(first), Some(last)) => format!("{}{}", first.as_str(), last.as_str()).parse::<i32>().unwrap(),
            }
        })
        .sum::<i32>());

    Ok(())
}
