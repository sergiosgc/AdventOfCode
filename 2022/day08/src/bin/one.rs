use std::{io::BufRead, collections::HashMap};
use aoc::coord::Coord;
use itertools::Itertools;
fn visibility_map(height_map: HashMap::<Coord, i64>, inner_increment: Coord) -> Vec<Coord> {
    let mut result = Vec::<Coord>::new();
    let size = height_map.clone().into_iter().map(|(coord, _height)| coord.x).reduce(|a,b| a.max(b)).unwrap() + 1;
    let start = if inner_increment.x < 0 || inner_increment.y < 0 { size - 1 } else { 0 };
    let outer_increment = Coord::new(inner_increment.y, inner_increment.x);
    let mut current = Coord::new(start, start);
    while current.x >= 0 && current.y >= 0 && current.x < size && current.y < size {
        let mut max_height: i64 = -1;
        let outer_current = current.clone();
        while (inner_increment.x == 0 && current.y >= 0 && current.y < size) ||
              (inner_increment.y == 0 && current.x >= 0 && current.x < size) {
                let height = *height_map.get(&current).unwrap();
                if height > max_height { result.push(current.clone()); }
                max_height = max_height.max(height);
                current = current + inner_increment.clone();
              }
        current = outer_current + outer_increment.clone();
    }
    result
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let height_map = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .enumerate()
        .map(|(line_no, line)| line
            .chars()
            .enumerate()
            .map(|(col_no, height)| ((col_no as i64, line_no as i64), height.to_string().parse::<i64>().unwrap()) )
            .collect::<Vec<((i64, i64), i64)>>()
        )
        .flatten()
        .fold(HashMap::<Coord, i64>::new(), |mut height_map, entry| { height_map.insert(Coord::new(entry.0.0, entry.0.1), entry.1); height_map });
    println!("{}", [
            visibility_map(height_map.clone(), Coord::new(1,0)),
            visibility_map(height_map.clone(), Coord::new(-1,0)),
            visibility_map(height_map.clone(), Coord::new(0,1)),
            visibility_map(height_map.clone(), Coord::new(0,-1)),
        ]
            .into_iter()
            .flatten()
            .unique()
            .count()
    );
    Ok(())
}
