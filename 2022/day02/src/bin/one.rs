use std::io::BufRead;
#[derive(Debug, PartialEq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors
}
#[derive(Debug)]
struct Round {
    mine: Move,
    opponent: Move
}
impl Round {
    fn from_string(s: String) -> Round {
        let split = s.split(" ").collect::<Vec<&str>>();
        Round { 
            mine: match split[1] {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => panic!("Unexpected")
            },
            opponent: match split[0] {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("Unexpected")
            },
        }
    }
    fn score(self) -> i64 {
        (match ( self.mine.clone(), self.opponent.clone() ) {
            (Move::Rock, Move::Scissors) | (Move::Scissors, Move::Paper) | (Move::Paper, Move::Rock) => 6,
            _ => if self.mine == self.opponent { 3 } else { 0 }
        }) + (match self.mine {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        })
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    println!("{:#?}", input.into_iter().map(|line| Round::from_string(line) ).map(|mve| mve.score()).sum::<i64>());
    Ok(())
}
