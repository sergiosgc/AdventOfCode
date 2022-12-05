use std::io::BufRead;
use itertools::Itertools;
use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (stack_input, move_input) = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok).collect::<Vec<String>>()
        .split(|line| line.len() == 0)
        .map(|slice| slice.into_iter().map(|s| s.clone() ).collect::<Vec<String>>())
        .next_tuple::<(Vec<String>, Vec<String>)>()
        .unwrap();
    let stacks = stack_input
        .into_iter()
        .filter(|line| line.find('[').is_some())
        .map(|line| line
            .replace("    ", "[_] ")
            .replace("[", "")
            .replace("]", "")
            .replace(" ", "")
            .chars()
            .collect::<Vec<char>>())
        .fold(Vec::<Vec<char>>::new(), |mut acc, line| {
            for (i,ch) in line.into_iter().enumerate() {
                while i+1 > acc.len() { acc.push(Vec::<char>::new()); }
                acc[i].push(ch);
            }
            acc

        })
        .into_iter()
        .map(|stack| stack.into_iter().filter(|ch| ch != &'_').collect::<Vec<char>>())
        .map(|mut stack| { stack.reverse(); stack })
        .collect::<Vec<Vec<char>>>()
        ;
    let moves = move_input
        .into_iter()
        .map(|line| {
            let captures = Regex::new(r"[^0-9]*(\d+)[^0-9]*(\d+)[^0-9]*(\d+)")
                .unwrap()
                .captures(&line)
                .unwrap();
            (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(), 
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1
            )
        })
        .collect::<Vec<(i64, usize, usize)>>();
    println!("{:#?}", moves
        .into_iter()
        .fold(stacks, |mut stack, (count, from, to)| {
            for _iteration in 0..count { 
                let elm = stack[from].pop().unwrap();
                stack[to].push(elm);
            }
            stack
        })
        .into_iter()
        .map(|mut stack| stack.pop().ok_or(' ').unwrap() )
        .collect::<String>()
        .replace(" ", "")
    );
    Ok(())
}
