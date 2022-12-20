use std::{io::BufRead};
const DECRIPTION_KEY: i64 = 811589153;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file: Vec<i64> = std::io::BufReader::new(std::io::stdin()).lines()
        .filter_map(std::io::Result::ok)
        .map(|line| line.parse::<i64>().unwrap() * DECRIPTION_KEY)
        .collect();
    let modulus = file.len() - 1;
    let mut workbench: Vec<(usize, i64)> = file.clone().into_iter().enumerate().collect();
    for _j in 0..10 {
        for i in 0..modulus+1 {
            let pos = workbench.iter().position(|(original_index, _value)| original_index == &i).unwrap();
            let new_pos = (pos as i64+ workbench[pos].1).rem_euclid(modulus as i64) as usize;
            let value = workbench[pos];
            workbench.remove(pos);
            workbench.insert(new_pos, value);
        }
    }
    let zero_pos: i64 = workbench.iter().enumerate().find(|(_i, value)| value.1 == 0).unwrap().0 as i64;
    println!("{:#?}", [
        workbench[(1000_i64 + zero_pos).rem_euclid(modulus as i64 +1) as usize].1,
        workbench[(2000_i64 + zero_pos).rem_euclid(modulus as i64 +1) as usize].1,
        workbench[(3000_i64 + zero_pos).rem_euclid(modulus as i64 +1) as usize].1,
    ].into_iter().sum::<i64>());
    Ok(())
}