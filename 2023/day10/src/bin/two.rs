use std::{io::BufRead, collections::HashMap};
use aoc::coord::Coord;
pub fn fill_distances(diagram: &HashMap<Coord, Vec<Coord>>, distances: &mut HashMap<Coord, i64>) -> () {
    let mut work_queue = distances.keys().map(Coord::clone).collect::<Vec<Coord>>();
    while !work_queue.is_empty() {
        let mut next_work_queue: Vec<Coord> = Vec::new();
        for pos in work_queue.into_iter() {
            let distance = distances.get(&pos).unwrap() + 1;
            diagram
            .get(&pos)
            .unwrap()
            .iter()
            .for_each(|direction| {
                let neighbour = pos + *direction;
                if distances.get(&neighbour).unwrap_or(&i64::MAX) > &distance {
                    distances.insert(neighbour.clone(), distance);
                    next_work_queue.push(neighbour.clone());
                }
            });
        }
        work_queue = next_work_queue.clone();
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut diagram, mut distances) = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .enumerate()
    .map(|(y, line)| {
        let mut diagram: HashMap<Coord, Vec<Coord>> = HashMap::new();
        let mut distances: HashMap<Coord, i64> = HashMap::new();
        line
        .chars()
        .enumerate()
        .for_each(|(x, ch)| {
            let connections = match ch {
                '|' => vec![Coord::new(0, -1), Coord::new(0, 1)],
                '-' => vec![Coord::new(-1, 0), Coord::new(1, 0)],
                'L' => vec![Coord::new(0, -1), Coord::new(1, 0)],
                'J' => vec![Coord::new(0, -1), Coord::new(-1, 0)],
                '7' => vec![Coord::new(0, 1), Coord::new(-1, 0)],
                'F' => vec![Coord::new(0, 1), Coord::new(1, 0)],
                _ => vec![],
            };
            if !connections.is_empty() {
                diagram.insert(Coord::new(x.try_into().unwrap(), y.try_into().unwrap()), connections);
            }
            if ch == 'S' {
                distances.insert(Coord::new(x.try_into().unwrap(), y.try_into().unwrap()), 0);
            }
        });
        (diagram, distances)
    })
    .reduce(|(mut diagram, mut distances), (item_diagram, item_distances)| {
        item_diagram.into_iter().for_each(|(k, v)| { diagram.insert(k, v); });
        item_distances.into_iter().for_each(|(k, v)| { distances.insert(k, v); });
        (diagram, distances)
    })
    .unwrap();
    // Connect the start point
    distances
    .clone()
    .into_iter()
    .for_each(|(start, _)| {
        diagram.insert(
            start, 
            vec![Coord::new(1,0), Coord::new(-1, 0), Coord::new(0, 1), Coord::new(0, -1)]
            .iter()
            .filter(|direction| diagram.contains_key(&(start + **direction)) && diagram.get(&(start + **direction)).unwrap().contains(&(-(**direction))))
            .map(|direction| direction.clone())
            .collect::<Vec<Coord>>());
    });
    fill_distances(&diagram, &mut distances);
    let mut inside_count: i64 = 0;
    for y in distances.keys().map(|coord| coord.y).min().unwrap()..distances.keys().map(|coord| coord.y).max().unwrap()+1 {
        let mut inside: bool = false;
        for x in distances.keys().map(|coord| coord.x).min().unwrap()..distances.keys().map(|coord| coord.x).max().unwrap()+1 {
            if distances.contains_key(&Coord::new(x,y)) {
                if let Some(connections) = diagram.get(&Coord::new(x,y)) {
                    if connections.contains(&Coord::new(0, 1)) {
                        inside = !inside;
                    }
                }
            } else if inside {
                inside_count += 1;
            }
        }
    }
    println!("{:#?}", inside_count);
    Ok(())
}
