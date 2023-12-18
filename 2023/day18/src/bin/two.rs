use std::{io::BufRead, collections::HashSet};

use aoc::coord::Coord;
pub fn print_map(map: &HashSet<Coord>) {
    let dimensions = Coord {
        x: map.iter().map(|c| c.x).max().unwrap(),
        y: map.iter().map(|c| c.y).max().unwrap(),
    };
    for y in 0..dimensions.y + 1 {
        for x in 0..dimensions.x+1 {
            print!("{}", if map.contains(&Coord { x, y }) { "#" } else { "." });
        }
        println!("");
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vertexes = vec![Coord::new(0,0)];
    let mut input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .map(|line| {
        let mut iter = line.split(" ");
        _ = iter.next();
        _ = iter.next();
        let mut length = iter.next().unwrap().replace(&['(', ')', '#'], "");
        ( 
            match length.split_off(5).as_str() {
                "0" => Coord::new(1,0),
                "1" => Coord::new(0,1),
                "2" => Coord::new(-1,0),
                _ => Coord::new(0,-1),

            },
            i64::from_str_radix(&length, 16).unwrap()
        )

    })
    .fold(&mut vertexes, |acc: &mut Vec<Coord>, (direction, length)| {
        let mut current = acc[acc.len() -1 ];
        acc.push(current + direction * length);
        acc
    });
    let to_shift = Coord {
        x: -input.iter().map(|c| c.x).min().unwrap() + 1,
        y: -input.iter().map(|c| c.y).min().unwrap() + 1,
    };
    *input = input.iter().map(|c| *c + to_shift).collect::<Vec<Coord>>();
    println!("{:?}", input.windows(2)
    .map(|v| (v[0], v[1]))
    .map(|(a,b)| a.x * b.y - a.y * b.x)
    .sum::<i64>() / 2
    +
    input.windows(2)
    .map(|v| (v[0], v[1]))
    .map(|(a,b)| (a.x - b.x).abs() + (a.y - b.y).abs())
    .sum::<i64>() / 2
    +1
    );
    Ok(())
}
