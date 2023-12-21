use std::{io::BufRead, collections::HashSet};
use aoc::coord::Coord;
use polynomial::Polynomial;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Garden {
    pub rocks: HashSet<Coord>,
    pub dimension: Coord,
}

pub fn reachable(garden: &Garden, from: &HashSet<Coord>, steps: i64) -> HashSet<Coord> {
    if steps == 0 {
        from.clone()
    } else {
        reachable(
            garden,
            &from.iter()
                .flat_map(Coord::neighbours)
                .filter(|c| garden.rocks.get(c).is_none())
                .collect()
            ,
            steps - 1
        )
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut start_pos = Coord { x: 0, y: 0 };
    let mut dimension = Coord { x: 0, y: 0 };
    let input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .enumerate()
    .flat_map(|(y, line)| vec![
        (y, format!("{}{}{}", line.replace("S", "."), line.replace("S", "."), line.replace("S", "."))),
        (y+line.len(), format!("{}{}{}", line.replace("S", "."), line, line.replace("S", "."))),
        (y+2*line.len(), format!("{}{}{}", line.replace("S", "."), line.replace("S", "."), line.replace("S", "."))),
    ])
    .flat_map(|(y, line)| line.chars().enumerate()
        .map(|(x, ch)| { 
            if ch == 'S' { start_pos = Coord{ x: x.try_into().unwrap(), y: y.try_into().unwrap(), }; }
            dimension = Coord::bounding_max(&dimension, &Coord{ x: x.try_into().unwrap(), y: y.try_into().unwrap(), });
            (x, ch) 
        })
        .filter(|(_, ch)| *ch == '#')
        .map(|(x, _)| Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap() })
        .collect::<Vec<Coord>>()
    )
    .collect::<HashSet<Coord>>();
    let samples = vec![
        (dimension.x - 3)/6 + 1,
        (dimension.x - 3)/6 + 1 + (1+(dimension.x)/3),
        (dimension.x - 3)/6 + 1 + 2*(1+(dimension.x)/3),
    ];
    let reachable = samples.clone().into_iter()
    .map(|iterations| reachable(&Garden { rocks: input.clone(), dimension }, &HashSet::from([start_pos]), iterations).iter().len() as i64)
    .collect::<Vec<i64>>();
    let poly = Polynomial::lagrange(&samples, &reachable).unwrap();
    println!("{:?}", poly.eval(26501365));
    Ok(())
}
