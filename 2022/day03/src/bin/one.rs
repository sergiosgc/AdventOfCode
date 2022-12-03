use std::{io::BufRead, collections::HashSet};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:#?}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| vec![line[0..line.len() as usize / 2].to_string(), line[line.len() as usize / 2..line.len() as usize].to_string()])
        .map(|compartments| compartments
            .into_iter()
            .map( |s| HashSet::<char>::from_iter(s.chars()) )
        )
        .map(|mut compartments| compartments
            .next()
            .unwrap()
            .intersection(&compartments.next().unwrap())
            .map(|ch| 
                *ch as i64 
                - (if (*ch as i64) < ('a' as i64) { -27 + 'A' as i64 } else { -1 + 'a' as i64 })
            )
            .sum::<i64>())
        .sum::<i64>()
    );
    Ok(())
}
