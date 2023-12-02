use std::io::BufRead;
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bag_content = (12i64, 13i64, 14i64);
    let line_re = Regex::new(r#"^Game (?P<id>\d+): (?P<sets>.*)$"#).unwrap();
    let set_re = Regex::new(r#"(?P<count>\d+) (?P<color>red|green|blue)(?:, )?"#).unwrap();
    println!("{:#?}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            let captures = line_re.captures(&line).unwrap();
            match (captures.name("id"), captures.name("sets")) {
                (Some(id), Some(sets)) => (
                    id.as_str().parse::<i64>().unwrap(),
                    sets.as_str()
                        .split("; ")
                        .into_iter()
                        .map(|set| set_re
                            .captures_iter(set)
                            .map(|capture| match capture.name("color").unwrap().as_str() {
                                "red" => (capture.name("count").unwrap().as_str().parse::<i64>().unwrap(), 0, 0),
                                "green" => (0, capture.name("count").unwrap().as_str().parse::<i64>().unwrap(), 0),
                                "blue" => (0, 0, capture.name("count").unwrap().as_str().parse::<i64>().unwrap()),
                                _ => panic!("Unknown color")
                            })
                            .fold((0, 0, 0), |acc, item| (acc.0 + item.0, acc.1 + item.1, acc.2 + item.2))
                        )
                        .fold((0,0,0), |acc, item| (acc.0.max(item.0), acc.1.max(item.1), acc.2.max(item.2)))
                    ),
                (_, _) => panic!("Impossible capture"),
            }
        })
        .map(|game| game.1.0 * game.1.1 * game.1.2)
        .sum::<i64>());
    Ok(())
}
