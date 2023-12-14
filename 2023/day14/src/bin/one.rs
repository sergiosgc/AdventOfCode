use std::{io::BufRead, collections::HashMap};
use aoc::coord::Coord;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Rock {
    #[default]
    Square,
    Round
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Platform {
    pub size: Coord,
    pub rocks: HashMap<Coord, Rock>
}
impl std::hash::Hash for Platform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        let mut rocks = self.rocks.iter().collect::<Vec<(&Coord, &Rock)>>();
        rocks.sort_by(|a,b| a.0.cmp(b.0));
        rocks.hash(state);
    }
}
impl Platform {
    pub fn tilt(&mut self) {
        for x in 0..self.size.x+1 {
            let mut squares = self.rocks.iter()
            .filter(|(coord, rock)| coord.x == x && **rock == Rock::Square)
            .map(|c| c.0.y)
            .collect::<Vec<i64>>();
            squares.push(-1);
            squares.push(self.size.y+1);
            squares.sort();
            let new_rocks = squares
            .windows(2)
            .map(|v| (v[0], v[1]))
            .flat_map(|(min, max)|
                (min+1..min+1+self.rocks.iter()
                .filter(|(coord, rock)| coord.x == x && coord.y > min && coord.y < max && **rock == Rock::Round)
                .count() as i64).collect::<Vec<i64>>()
            )
            .map(|y| (Coord { x, y }, Rock::Round) )
            .collect::<Vec<(Coord, Rock)>>();
            let old_rocks = self.rocks.iter()
                .filter(|(coord, rock)| coord.x == x && **rock == Rock::Round)
                .map(|c| c.0.clone())
                .collect::<Vec<Coord>>();
            for coord in old_rocks { self.rocks.remove(&coord); }
            for rock in new_rocks { self.rocks.insert( rock.0, rock.1 ); }
        }
    }
    pub fn load(self) -> i64 {
        self.rocks.iter()
        .filter(|(_, rock)| **rock == Rock::Round)
        .map(|(coord, _)| self.size.y - coord.y + 1)
        .sum::<i64>()
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = {
        let input = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .enumerate()
        .flat_map(|(y, line)| {
            line
            .chars()
            .enumerate()
            .map(|(x, ch)| {
                (
                    Coord { x: x.try_into().unwrap(), y: y.try_into().unwrap() },
                    match ch {
                        '#' => Some(Rock::Square),
                        'O' => Some(Rock::Round),
                        _ => None
                    }
                )
            })
            .collect::<Vec<(Coord, Option<Rock>)>>()
        })
        .collect::<Vec<(Coord, Option<Rock>)>>();
        Platform {
            size: input
                  .iter()
                  .map(|v| v.0)
                  .reduce(|a, b| Coord::bounding_max(&a, &b))
                  .unwrap(),
            rocks: input
                   .iter()
                   .filter(|(_, rock)| rock.is_some())
                   .map(|(coord, rock)| (coord.clone(), rock.unwrap().clone()))
                   .collect()
        }
    };
    input.tilt();
    println!("{:#?}", input.load());
    Ok(())
}