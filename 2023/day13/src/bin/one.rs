use std::{io::BufRead, collections::HashSet};
use aoc::coord::Coord;
use itertools::Itertools;

pub fn is_range_symmetric(line: &HashSet<Coord>, x: i64, max_x: i64) -> bool {
    if (max_x - x + 1) % 2 != 0 {
        false
    } else {
        let mut to_test = line.iter().filter(|c| c.x >= x && c.x <= max_x).collect::<Vec<&Coord>>();
        if to_test.len() % 2 != 0 {
            false
        } else {
            to_test.sort_by(|a,b| a.x.cmp(&b.x));
            to_test
            .iter()
            .zip(
                to_test
                .iter()
                .rev()
            )
            .map(|(a,b)| (true, a.x + b.x))
            .reduce(|a,b| (a.0 && a.1 == b.1, a.1))
            .unwrap_or((true, 0)).0
        }

    }
}
pub fn horizontal_simmetry(puzzle: &Vec<HashSet<Coord>>) -> Option<f64> {
    let max_x = puzzle
    .iter()
    .flat_map(|line| line.iter().collect::<Vec<&Coord>>())
    .map(|c| c.x)
    .max()
    .unwrap();
    match
        puzzle
        .iter()
        .map(|line| (1..max_x)
            .cartesian_product(vec![0, max_x].into_iter())
            .map(|(a, b)| (a.min(b), a.max(b)))
            .filter(|(x1, x2)| is_range_symmetric(line, *x1, *x2)).collect::<HashSet<(i64, i64)>>() )
        .collect::<Vec<HashSet<(i64, i64)>>>()
        .into_iter()
        .reduce(|a,b| a.intersection(&b).map(|v| v.clone()).collect::<HashSet<(i64, i64)>>())
        .unwrap_or(HashSet::<(i64, i64)>::new())
        .iter()
        .next() {
            Some((x1, x2)) => Some((x1 + x2) as f64 / 2_f64),
            None => None
        }
}
pub fn rotate(puzzle: &Vec<HashSet<Coord>>) -> Vec<HashSet<Coord>> {
    let max_x = puzzle
    .into_iter()
    .flat_map(|line| 
        line
        .iter()
        .collect::<Vec<&Coord>>())
    .map(|c| c.x)
    .max()
    .unwrap();
    let rotated = puzzle
    .into_iter()
    .flat_map(|line| 
        line
        .iter()
        .collect::<Vec<&Coord>>())
    .map(|c| Coord { x: c.y, y: -c.x + max_x})
    .collect::<Vec<Coord>>();
    let max_y = rotated
    .clone()
    .into_iter()
    .map(|c| c.y)
    .max()
    .unwrap();
    let result = (0..max_y+1).map(|_| HashSet::<Coord>::new()).collect::<Vec<HashSet<Coord>>>();
    rotated
    .into_iter()
    .fold(result, |mut acc: Vec<HashSet<Coord>>, rotated| {
        acc[(rotated.y) as usize].insert(rotated);
        acc
    })
}
pub fn print_puzzle(puzzle: &Vec<HashSet<Coord>>) -> () {
    let max = puzzle
    .into_iter()
    .flat_map(|line| 
        line
        .iter()
        .collect::<Vec<&Coord>>())
    .map(|c| *c)
    .reduce(|a, b| Coord { x: a.x.max(b.x), y: a.y.max(b.y) })
    .unwrap();

    (0..(max.y+1) as usize).cartesian_product(0..(max.x+1) as usize).for_each(|(y, x)| {
        if x == 0 { println!("") }
        print!("{}", match puzzle[y].contains(&Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap() }) {
            true => '#',
            false => '.'
        });
    });
    println!("");
    ()
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .fold(&mut Vec::<(i64, String)>::new(), |acc, line| {
        let current = match acc.len() { 0 => 0, _ => acc.get(acc.len()-1).unwrap().0 };
        acc.push( (if line.is_empty() { current + 1 } else { current }, line) );
        acc
    })
    .iter()
    .filter(|line| !line.1.is_empty() )
    .group_by(|line| line.0)
    .into_iter()
    .map(|(_, puzzle)| {
        puzzle
        .into_iter()
        .enumerate()
        .map(|(y, (_, line))| 
            line
            .chars()
            .enumerate()
            .flat_map(|(x, ch)| match ch { '#' => Some(Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap() }), _ => None } )
            .collect::<HashSet<Coord>>() )
        .collect::<Vec<HashSet<Coord>>>()
    })
    .collect::<Vec<Vec<HashSet<Coord>>>>();
    
    println!("{:#?}", input.iter().map(|puzzle| horizontal_simmetry(puzzle)).map(|v| match v { Some(v) => v.ceil() as i64, None => 0 } ).zip(
                        input.iter().map(rotate).map(|puzzle| horizontal_simmetry(&puzzle)).map(|v| match v { Some(v) => 100 * v.ceil() as i64, None => 0 } )
                    ).map(|(a,b)| a+b).sum::<i64>());
    Ok(())
}
