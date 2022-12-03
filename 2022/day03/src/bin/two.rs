use std::{io::BufRead, collections::HashSet};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:#?}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| HashSet::<char>::from_iter(line.chars()) )
        .fold(Vec::<Vec::<HashSet::<char>>>::new(), |mut acc, line| {
            let peek = if acc.len() > 0 { acc.len() - 1 } else { 0 };
            if acc.len() == 0 || acc[peek].len() == 3 {
                acc.push(vec![line]);
            } else {
                acc[peek].push(line);
            }
            acc
        })
        .into_iter()
        .map(|group| HashSet::<char>::from_iter(group[0].intersection(&group[1])
            .map(|c| (*c).clone() ))
            .intersection(&group[2])
            .map(|ch| 
                *ch as i64 
                - (if (*ch as i64) < ('a' as i64) { -27 + 'A' as i64 } else { -1 + 'a' as i64 })
            )
            .sum::<i64>()
        ).sum::<i64>()
    );
    Ok(())
}