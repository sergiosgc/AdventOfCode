use std::{io::BufRead};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file: Vec<i64> = std::io::BufReader::new(std::io::stdin()).lines()
        .filter_map(std::io::Result::ok)
        .flat_map(|line| line.parse::<i64>())
        .collect();
    let modulus = file.len() - 1;
    let mut workbench: Vec<(i64, bool)> = file.clone().into_iter().map(|v| (v, false) ).collect();
    let mut pos = 0;
    for _i in 0..modulus+1 {
        while workbench[pos].1 { pos += 1; }
        let new_pos = (pos as i64+ workbench[pos].0).rem_euclid(modulus as i64) as usize;
        let value = workbench[pos].0;
        workbench.remove(pos);
        workbench.insert(new_pos, (value, true));
    }
    let zero_pos: i64 = workbench.iter().enumerate().find(|(_i, (value, _processed))| *value == 0).unwrap().0 as i64;
    println!("{:#?}", [
        workbench[(1000_i64 + zero_pos).rem_euclid(modulus as i64 +1) as usize].0,
        workbench[(2000_i64 + zero_pos).rem_euclid(modulus as i64 +1) as usize].0,
        workbench[(3000_i64 + zero_pos).rem_euclid(modulus as i64 +1) as usize].0,
    ].into_iter().sum::<i64>());
    Ok(())
}
