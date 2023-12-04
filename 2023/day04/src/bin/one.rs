use std::{io::BufRead, collections::{HashSet, hash_map::RandomState}};
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line_re = Regex::new(r#"^Card *(?P<id>\d+): (?P<numbers>.*)$"#).unwrap();
    println!("{:#?}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            let captures = line_re.captures(&line).unwrap();
            match (captures.name("id"), captures.name("numbers")) {
                (Some(id), Some(numbers)) => (
                    id.as_str().parse::<i64>().unwrap(),
                    numbers.as_str()
                        .split("| ")
                        .into_iter()
                        .map(|sequence| HashSet::<i64, RandomState>::from_iter(
                            sequence
                            .split(" ")
                            .into_iter()
                            .flat_map(|value| value.parse::<i64>())
                            .collect::<Vec<i64>>()
                            .into_iter()
                        )
                    )
                    .collect::<Vec<HashSet::<i64, RandomState>>>()
                ),
                (_,_) => panic!("Impossible capture")
            }
        })
        .map(|(_id, numbers)| numbers[0].intersection(&numbers[1]).count() as u32)
        .filter(|n| n > &0)
        .map(|n| 2_i64.pow(n-1))
        .sum::<i64>()
    );
    Ok(())
}
