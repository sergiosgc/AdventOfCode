use std::{io::BufRead, collections::{HashSet, hash_map::RandomState}};
use regex::Regex;
pub fn count_cards(mut cards: Vec<(i64, i64, i64)>) -> i64 {
    if cards.len() == 1 {
        cards[0].1
    } else {
        let current = cards[0];
        for n in 1_usize..(1_usize + <i64 as TryInto<usize>>::try_into(current.2).unwrap()) {
            cards[n].1 += current.1;
        }
        current.1 + count_cards(cards.split_off(1))
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let line_re = Regex::new(r#"^Card *(?P<id>\d+): (?P<numbers>.*)$"#).unwrap();
    println!("{:#?}", count_cards(std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            let captures = line_re.captures(&line).unwrap();
            match (captures.name("id"), captures.name("numbers")) {
                (Some(id), Some(numbers)) => (
                    id.as_str().parse::<i64>().unwrap(),
                    1_i64,
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
                    .iter()
                    .fold(None, |acc: Option<HashSet::<i64, RandomState>>, item| Some(if acc.is_none() { item.clone() } else { HashSet::from_iter( acc.unwrap().intersection(&item).map(|v| v.clone()) )}))
                    .unwrap()
                    .len() as i64
                ),
                (_,_) => panic!("Impossible capture")
            }
        })
        .collect::<Vec<(i64, i64, i64)>>()
    ));
    Ok(())
}