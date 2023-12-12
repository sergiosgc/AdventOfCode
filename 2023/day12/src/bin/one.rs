use std::{io::BufRead, collections::HashMap};
pub fn group_pattern(s: &String) -> Vec<i64> {
    let mut result = Vec::<i64>::new();
    for ch in s.chars() {
        let current = result.pop().unwrap_or(0);
        if ch == '.' {
            if current != 0 {
                result.push(current);
                result.push(0);
            } else {
                result.push(current);
            }
        } else if ch == '#' {
            result.push(current + 1)
        } else if ch == '?' {
            result.push(current);
            {
                let peek = result.pop().unwrap_or(0);
                if peek != 0 { result.push(peek); }
            }
            return result;
        }
    }
    let peek = result.pop().unwrap_or(0);
    if peek != 0 { result.push(peek); }
    result
}
pub fn partial_pattern_fits(partial: &Vec<i64>, complete: &Vec<i64>) -> (bool, i64) {
    let remaining = complete.iter().sum::<i64>() - partial.iter().sum::<i64>(); 
    if partial.len() > complete.len() {
        (false, remaining)
    } else if partial.len() == complete.len() {
        (partial.into_iter().zip(complete.into_iter()).find(|(a,b)| **a > **b).is_none(), remaining)
    } else {
        for delta in 0..complete.len()-partial.len() {
            if partial.len() == 0 {
                return (true, remaining);
            } else if partial.len() == 1 {
                if partial[0] <= complete[delta] {
                    return (true, remaining);
                }
            } else {
                if partial.iter().take(partial.len()-1).zip(complete.iter().skip(delta)).find(|(a,b)| **a != **b).is_none()
                && partial[partial.len()-1] <= complete[delta + partial.len()-1] {
                    return (true, remaining);
                }
            }
        }
        (false, remaining)
    }
}
pub fn permutations(record: &mut String, groups: &Vec<i64>, memoization: &mut HashMap<(Vec<i64>, String), i64>) -> i64 {
    let partial_groups = group_pattern(record);
    let (partial_fit, remaining) = partial_pattern_fits(&partial_groups, groups);
    if !partial_fit { return 0; }
    if let Some(unknown_idx) = record.find("?") {
        if remaining > (record.len()-unknown_idx) as i64 { return 0; }
        if let Some(result) = match unknown_idx { 0 => None, _ => memoization.get(&(partial_groups.clone(), record.clone().split_off(unknown_idx-1))) } {
            *result
        } else {
            let mut result = 0;
            record.replace_range(unknown_idx..unknown_idx+1, ".");
            result += permutations(record, groups, memoization);
            record.replace_range(unknown_idx..unknown_idx+1, "#");
            result += permutations(record, groups, memoization);
            record.replace_range(unknown_idx..unknown_idx+1, "?");
            if unknown_idx > 0 { memoization.insert((partial_groups, record.clone().split_off(unknown_idx-1)), result); }
            result
        }
    } else {
        if partial_groups.len() != groups.len() {
            0
        } else {
            match partial_groups.iter().zip(groups.iter()).find(|(a,b)| **a != **b) {
                Some(_) => 0,
                None => 1
            }
        }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .map(|line| {
        let (damaged, groups) = line.split_once(" ").unwrap();
        ( damaged.to_string(), groups.replace(&['(', ')'], "").split(",").flat_map(str::parse::<i64>).collect::<Vec<i64>>() )
    })
    .collect::<Vec<(String, Vec<i64>)>>();
    println!("{:#?}", input.iter().map(|record| permutations(&mut record.clone().0, &record.1, &mut HashMap::new())).sum::<i64>());
    Ok(())
}
