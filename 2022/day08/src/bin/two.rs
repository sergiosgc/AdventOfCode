use std::{io::BufRead, collections::HashMap};
use aoc::coord::Coord;
fn viewing_distance(height_map: HashMap::<Coord, i64>, origin: Coord, increment: Coord) -> i64 {
    let size = height_map.clone().into_iter().map(|(coord, _height)| coord.x).reduce(|a,b| a.max(b)).unwrap() + 1;
    let mut current = origin.clone() + increment.clone();
    let my_height = *height_map.get(&origin).unwrap();
    let mut result = 0;
    while current.x >= 0 && current.y >= 0 && current.x < size && current.y < size && *height_map.get(&current).unwrap() < my_height {
        result += 1;
        current = current + increment.clone();
    }
    if current.x >= 0 && current.y >= 0 && current.x < size && current.y < size { result += 1; }
    result
}
fn scenic_score(height_map: HashMap::<Coord, i64>, origin: Coord) -> i64 {
    viewing_distance(height_map.clone(), origin.clone(), Coord::new(0,-1))
    * viewing_distance(height_map.clone(), origin.clone(), Coord::new(0,1))
    * viewing_distance(height_map.clone(), origin.clone(), Coord::new(1,0))
    * viewing_distance(height_map.clone(), origin.clone(), Coord::new(-1,0))
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
    println!("{}", height_map.clone().into_iter().map(|(coord, _height)| scenic_score(height_map.clone(), coord.clone()) ).reduce(|a,b| a.max(b)).unwrap());

    Ok(())
}
