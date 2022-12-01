use std::io::BufRead;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut elfs = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .collect::<Vec<String>>()
        .split(|line| line.len() == 0)
        .map(|slice| slice.to_vec())
        .map(|elf_calories| elf_calories
            .into_iter()
            .fold(0, |result, calorie| result + calorie.parse::<i64>().unwrap() ))
        .collect::<Vec<i64>>();
    elfs.sort();
    elfs.reverse();
    println!("{:#?}", elfs[0..3].into_iter().sum::<i64>());
    Ok(())
}
