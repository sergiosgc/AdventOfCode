use std::{io::BufRead, collections::HashMap};

use aoc::{coord::Coord, elf::Elf};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin()).lines()
        .filter_map(std::io::Result::ok)
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter(|(_x, ch)| *ch == '#' )
            .map(|(x, _ch)| Coord{ x: x as i64, y: y as i64 } )
            .collect::<Vec<Coord>>()
        ).fold(HashMap::<Coord, Elf>::new(), |mut grove, coord| {
            grove.insert(coord, Elf { ..Default::default() });
            grove
        });
    for round in 0..10_i64 {
        let move_coords = input
            .iter()
            .fold(HashMap::<Coord, Vec<Coord>>::new(), |mut next_positions, (elf_coord, elf)| {
                match elf.plan_move(round, elf_coord, &input.keys().copied().collect::<Vec<Coord>>()) {
                    None => {},
                    Some(next_position) => {
                        if let std::collections::hash_map::Entry::Vacant(e) = next_positions.entry(next_position) {
                            e.insert(vec![*elf_coord]);
                        } else {
                            next_positions.get_mut(&next_position).unwrap().push(*elf_coord);
                        }
                    }
                };
                next_positions
            }).iter()
            .filter(|(_to_coord, from_coords)| from_coords.len() == 1)
            .map(|(to_coord, from_coords)| (*to_coord, from_coords[0]))
            .collect::<Vec<(Coord, Coord)>>();
        for (to_coord, from_coord) in move_coords {
            let elf = input.remove(&from_coord).unwrap();
            input.insert(to_coord, elf);
        }
    }
    let min_coord = Coord {
        x: input.keys().map(|coord| coord.x).min().unwrap(),
        y: input.keys().map(|coord| coord.y).min().unwrap(),
    };
    let max_coord = Coord {
        x: input.keys().map(|coord| coord.x).max().unwrap(),
        y: input.keys().map(|coord| coord.y).max().unwrap(),
    };
    println!("{:#?}", (max_coord.x - min_coord.x + 1) * (max_coord.y - min_coord.y + 1) - input.len() as i64);
    Ok(())
}
