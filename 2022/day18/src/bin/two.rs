use std::io::BufRead;
use aoc::{regex::ExtractTuple3, coord::Coord};

use regex::Regex;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let droplet: Vec<Coord> = std::io::BufReader::new(std::io::stdin())
        .lines().filter_map(std::io::Result::ok)
        .map(|line| Regex::new(r#"^(\d+),(\d+),(\d+)$"#).unwrap()
            .captures(&line).unwrap()
            .extract_tuple()
        )
        .map(|(x, y, z)| Coord { x, y, z })
        .collect();
    let droplet_adjacent = droplet.iter()
        .flat_map(|coord| coord.neighbours())
        .filter(|coord| !droplet.contains(coord))
        .collect::<Vec<Coord>>();
    let bounding_box_min = droplet_adjacent.clone().into_iter().reduce(|a, b| Coord { x: a.x.min(b.x),  y: a.y.min(b.y),  z: a.z.min(b.z) }).unwrap();
    let bounding_box_max = droplet_adjacent.clone().into_iter().reduce(|a, b| Coord { x: a.x.max(b.x),  y: a.y.max(b.y),  z: a.z.max(b.z) }).unwrap();
    let water = {
        let mut stack = vec![bounding_box_min];
        let mut result: Vec<Coord> = Vec::new();
        while !stack.is_empty() {
            let item = stack.pop().unwrap();
            result.push(item);
            stack.append(&mut item.neighbours()
                .into_iter()
                .filter(|neighbour| neighbour.x >= bounding_box_min.x && neighbour.y >= bounding_box_min.y && neighbour.z >= bounding_box_min.z)
                .filter(|neighbour| neighbour.x <= bounding_box_max.x && neighbour.y <= bounding_box_max.y && neighbour.z <= bounding_box_max.z)
                .filter(|neighbour| !droplet.contains(neighbour) && !result.contains(neighbour))
                .collect::<Vec<Coord>>()
            );
        }
        result
    };
    println!("{:?}", droplet_adjacent.iter().filter(|c| water.contains(c)).count());
    Ok(())
}
