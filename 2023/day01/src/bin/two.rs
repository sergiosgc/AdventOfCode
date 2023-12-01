use std::io::BufRead;
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r#"^[^0-9]*(?P<first>\d).*?(?P<last>\d)?[^0-9]*$"#).unwrap();
    println!("{}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            let mut result = line.clone();
            let re = Regex::new(r#"(one|two|three|four|five|six|seven|eight|nine)"#).unwrap();
            let mut offset: usize = 0;
            while offset < 1 + result.len() && re.is_match_at(&result, offset) {
                let re = Regex::new(&format!(r#"^(.{{{}}}.*?){}(.*)$"#, 
                    offset,
                    r#"(one|two|three|four|five|six|seven|eight|nine)"#)).unwrap();
                let captured_result = result.clone();
                let captures = re.captures(&captured_result).unwrap();
                result = format!("{}{}{}{}", 
                        captures.get(1).unwrap().as_str(),
                        match captures.get(2).unwrap().as_str() {
                            "one" => "1",
                            "two" => "2",
                            "three" => "3",
                            "four" => "4",
                            "five" => "5",
                            "six" => "6",
                            "seven" => "7",
                            "eight" => "8",
                            "nine" => "9",
                            &_ => panic!("Impossible text digit")
                        },
                        captures.get(2).unwrap().as_str(),
                        captures.get(3).unwrap().as_str());
                offset = captures.get(1).unwrap().as_str().len() + 2;
            }
            result
        })
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