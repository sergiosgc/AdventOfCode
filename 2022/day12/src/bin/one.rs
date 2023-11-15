// S->83, a->97

use std::io::BufRead;
use std::collections::HashMap;
use aoc::coord::Coord;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin()).lines()
        .filter_map(std::io::Result::ok)
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .map(|ch| ch.into())
            .map(|ch: u32| ch as i64 - 96)
            .enumerate()
            .map(move |(x, height)| (Coord{ x: x as i64, y: y as i64 }, height.clone()) )
            .collect::<Vec<(Coord, i64)>>()
        ).fold(HashMap::<Coord, i64>::new(), |mut map, item| {
            map.insert(item.0, item.1);
            map
        });
    let _input = input.clone();
    let start: Coord = match _input
        .iter()
        .find(|(&_coord, &height)| height == (83 - 96)) {
            None => panic!("Start not found"),
            Some(item) => item.0
        }.clone();
    let finish: Coord = match _input
        .iter()
        .find(|(&_coord, &height)| height == (69 - 96)) {
            None => panic!("Finish not found"),
            Some(item) => item.0
        }.clone();
    input.insert(start.clone(), 'a' as i64 - 96);
    input.insert(finish.clone(), 'z' as i64 - 96);
    let mut visited = HashMap::<Coord, i64>::new();
    visited.insert(start.clone(), 0);
    let mut queue: Vec<Coord> = vec![start.clone()];
    let directions = vec![Coord{ x: 1, y: 0 },Coord{ x: -1, y: 0 },Coord{ x: 0, y: 1 },Coord{ x: 0, y: -1 }];
    while queue.len() > 0 && visited.get(&finish).is_none() {
        let current = queue.remove(0);
        let mut valid_next = directions.clone()
            .into_iter()
            .map(|delta| current + delta)
            .filter(|c| visited.get(&c).is_none())
            .filter(|c| input.get(&c).is_some())
            .filter(|c| input.get(&c).unwrap() - input.get(&current).unwrap() < 2)
            .collect::<Vec<Coord>>()
            ;
        valid_next.iter().for_each(|c| { visited.insert(*c, visited.get(&current).unwrap() + 1); } );
        queue.append(&mut valid_next);
    }
    println!("{:#?}", visited.get(&finish));
    Ok(())
}
