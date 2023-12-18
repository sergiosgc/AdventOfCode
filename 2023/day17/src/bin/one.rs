use std::io::BufRead;
use aoc::{matrix::Matrix, coord::Coord};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct State {
    pub pos: Coord,
    pub direction: Coord,
    pub steps: usize,
}
pub fn successors(map: &Matrix<i64>, current: &State) -> Vec<(State, usize)> {
    let rotate_left = Coord { x: -current.direction.y, y: current.direction.x };
    let rotate_right = Coord { x: current.direction.y, y: -current.direction.x };
    let map_size: Coord = map.size().try_into().unwrap();
    vec![
        State {
            pos: current.pos + current.direction,
            direction: current.direction,
            steps: current.steps + 1,
        },
        State {
            pos: current.pos + rotate_left,
            direction: rotate_left,
            steps: 1,
        },
        State {
            pos: current.pos + rotate_right,
            direction: rotate_right,
            steps: 1,
        },
    ]
    .into_iter()
    .filter(|state| state.steps <= 3
        && state.pos.inside(&(0,0).try_into().unwrap(), &map_size))
    .map(|state| (state.clone(), map[state.pos.into()] as usize))
    .collect()
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .enumerate()
    .flat_map(|(y, line)| line.chars().enumerate().map(|(x, ch)| ((x,y), ch.to_string().parse::<i64>().unwrap())).collect::<Vec<((usize, usize), i64)>>())
    .collect::<Matrix<i64>>();
    let goal: Coord = input.size().try_into().unwrap();
    let result = dijkstra(
        &State { pos: Coord::new(0,0), direction: Coord::new(1,0), steps: 0 },
        |state| successors(&input, state),
        |state| state.pos == goal
    ).unwrap().1;
        
    println!("{:#?}", result);
    Ok(())
}
