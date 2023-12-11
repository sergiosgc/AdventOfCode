use std::{io::BufRead, collections::HashSet};
use itertools::Itertools;
use aoc::coord::Coord;
pub fn expanded_manhattan_distance(a: (Coord, (i64, i64)), b: (Coord, (i64, i64))) -> i64 {
    (a.0.x - b.0.x).abs() + (a.1.0 - b.1.0).abs() * (1000000-1) +
    (a.0.y - b.0.y).abs() + (a.1.1 - b.1.1).abs() * (1000000-1)
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = {
        let input = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .enumerate()
        .flat_map(|(y, line)| {
            line
            .chars()
            .enumerate()
            .flat_map(|(x, ch)| if ch == '#' {
                Some(Coord::new(x.try_into().unwrap(),y.try_into().unwrap()))
            } else {
                None
            })
            .collect::<Vec<Coord>>()

        })
        .collect::<HashSet<Coord>>();
        let top_left = input.clone().into_iter().reduce(|a,b| Coord::bounding_min(&a, &b)).unwrap();
        let bottom_right = input.clone().into_iter().reduce(|a,b| Coord::bounding_max(&a, &b)).unwrap();
        let mut empty_rows = (top_left.y..bottom_right.y).collect::<HashSet<i64>>();
        let mut empty_columns = (top_left.x..bottom_right.x).collect::<HashSet<i64>>();
        input
        .iter()
        .for_each(|coord| {
            empty_rows.remove(&coord.y);
            empty_columns.remove(&coord.x);
        });
        input
        .into_iter()
        .map(|coord| ( coord, ( 
                empty_columns
                .clone()
                .into_iter()
                .filter(|col| *col < coord.x)
                .count() as i64,
                empty_rows
                .clone()
                .into_iter()
                .filter(|row| *row < coord.y)
                .count() as i64
            )
        ))
        .collect::<Vec<(Coord, (i64, i64))>>()
    };
    println!("{:#?}", input.into_iter().combinations(2).map(|combination| expanded_manhattan_distance(combination[0], combination[1])).sum::<i64>());
    Ok(())
}
