use std::io::BufRead;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::ops::Add;

#[derive(Clone,Debug)]
struct Coord(i32, i32);
impl Coord {
    fn new(x: i32, y: i32) -> Coord { Coord{ 0: x, 1: y }} 
}
impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}
impl PartialEq<(i32, i32)> for Coord {
    fn eq(&self, other: &(i32, i32)) -> bool { return self.0 == other.0 && self.1 == other.1; }
    fn ne(&self, other: &(i32, i32)) -> bool { return self.0 != other.0 || self.1 != other.1; }
}
#[derive(Clone,Debug)]
struct BurrowState {
    energy: i64,
    burrow: [String; 7]
}
impl PartialEq for BurrowState {
    fn eq(&self, other: &Self) -> bool {
        for (i, line) in self.burrow.iter().enumerate() { if line != &other.burrow[i] { return false; }}
        return true;
    }
}
impl Eq for BurrowState {}
impl Hash for BurrowState {
    fn hash<H: Hasher>(&self, state: &mut H) { for line in self.burrow.clone() { line.hash(state); }}
}
impl Index<(i32, i32)> for BurrowState {
    type Output = str;
    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.burrow[index.1 as usize][index.0 as usize..index.0 as usize + 1]
    }
}
impl Index<Coord> for BurrowState {
    type Output = str;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.burrow[index.1 as usize][index.0 as usize..index.0 as usize + 1]
    }
}
impl BurrowState {
    fn parse(input: Vec<String>) -> BurrowState {
        BurrowState {
            energy: 0,
            burrow: [
                input[0].clone(),
                input[1].clone(),
                input[2].clone(),
                input[3].clone(),
                input[4].clone(),
                input[5].clone(),
                input[6].clone(),
            ]
        }
    }
    fn is_solution(state: &BurrowState) -> bool {
        for (x, amphipod) in [ (3, "A"), (5, "B"), (7, "C"), (9, "D")] { 
            for y in 2..state.burrow.len() as i32 - 1{
                if &state[(x,y)] != amphipod { return false; }
            }
        }
        return true;
    }
    fn solve(&self) -> BurrowState {
        let mut stack: Vec<BurrowState> = Vec::new();
        let mut visited: HashSet<BurrowState> = HashSet::new();
        stack.push(self.clone());
        loop {
            let mut current = stack.pop().unwrap();
            while match visited.get(&current) { Some(_b) => true, None => false } { current = stack.pop().unwrap() }
            visited.insert(current.clone());
            println!("{:#?}", current);
            if BurrowState::is_solution(&current) { return current; }
            let mut next: Vec<BurrowState> = current.gen_moves()
                .into_iter()
                .filter( |b| match visited.get(b) { Some(_b) => false, None => true })
                .collect();
            stack.append( &mut next );
            stack.sort_unstable_by(|a, b| b.energy.cmp(&a.energy));
        }
    }
    fn in_correct_position(&self, pos: Coord) -> bool {
        match &self[pos.clone()] {
            "A" | "B" | "C" | "D" => {},
            "#" => return true,
            _ => return false
        }
        if pos.0 != match &self[pos.clone()] {
            "A" => 3,
            "B" => 5,
            "C" => 7,
            "D" => 9,
            _ => -1
        } { return false; }
        return self.in_correct_position(pos + Coord { 0:0, 1:1 })
    }
    fn gen_moves(&self) -> Vec<BurrowState> {
        let mut result: Vec<BurrowState> = Vec::new();
        for y in 0..self.burrow.len() as i32 {
            for x in 0..self.burrow[0].len() as i32 {
                if self.in_correct_position(Coord { 0: x, 1: y}) { continue; }
                for delta in [Coord::new(-1,0), Coord::new(1,0), Coord::new(0,1), Coord::new(0, -1)] {
                    let topline_count = self.burrow[1].replace("#", "").replace(".", "").len();
                    result.append(&mut self.move_amphipod(Coord::new(x,y), delta)
                        .into_iter()
                        .map( |x| x.1 )
                        .filter( |b| -> bool {
                            for x in [ 3, 5, 7, 9 ] {
                                for y in 1..b.burrow.len() as i32 {
                                    match &b[(x,y)] {
                                        "A" | "B" | "C" | "D" => if &b[(x, y+1)] == "." || y == 1 { return false; }
                                        _ => {}
                                    }
                                }
                            }
                        true
                        })
                        .filter( |b| y != 1 || topline_count > b.burrow[1].replace("#", "").replace(".", "").len() )
                        .collect())
                }
            }
        }
        result
    }
    fn move_amphipod(&self, which: Coord, delta: Coord) -> Vec<(Coord, BurrowState)> {
        let mut result: Vec<(Coord, BurrowState)> = Vec::new();
        match &self[which.clone()] {
            "A" | "B" | "C" | "D" => { },
            _ => { return result; }
        }
        match &self[which.clone() + delta.clone()] {
            "." => {},
            _ => { return result; }
        }
        if delta == (0,1) && which.1 == 1 {
            if which.0 != match &self[which.clone()] {
                "A" => 3,
                "B" => 5,
                "C" => 7,
                "D" => 9,
                _ => panic!("Unexpected amphipod")
            } { return result; }
            for y in 0..self.burrow.len() as i32 {
                match &self[(which.0, y)] {
                    "." | "#" => {},
                    amphipod => if amphipod != &self[which.clone()] { return result; } 
                }
            }
        }
        let next = self._move_amphipod(which.clone(), which.clone() + delta.clone());
        result.push((Coord::new(0,0), next.clone()));
        match delta {
            Coord { 0:1, 1:0 } | Coord { 0:-1, 1:0 } => {
                result.append( &mut next.move_amphipod( which.clone() + delta.clone(), Coord::new(0,1) ) );
            },
            Coord { 0:0, 1:-1 } => {
                result.append( &mut next.move_amphipod( which.clone() + delta.clone(), Coord::new(1 ,0) ) );
                result.append( &mut next.move_amphipod( which.clone() + delta.clone(), Coord::new(-1,0) ) );
            }
            Coord { 0:0, 1:1 } => { },
            _ => panic!("Unexpected move_amphipod delta {:?}", delta)
        }
        result.append( &mut next.move_amphipod( which.clone() + delta.clone(), delta.clone() ) );
        result.into_iter()
            .map( |state| (state.0 + delta.clone(), state.1))
            .collect()
    }
    fn _move_amphipod(&self, from: Coord, to: Coord) -> BurrowState {
        let energy = match &self[from.clone()] {
            "A" => 1,
            "B" => 10,
            "C" => 100,
            "D" => 1000,
            _ => panic!("Unexpected target to move_amphipod")
        };
        let mut result = self.clone();
        result.energy += energy * (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as i64;
        result.burrow[to.1 as usize].replace_range(to.0 as usize..to.0 as usize + 1, &result[from.clone()].to_owned());
        result.burrow[from.1 as usize].replace_range(from.0 as usize..from.0 as usize + 1, ".");
        result
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: BurrowState = BurrowState::parse(std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect());
    println!("{:#?}", input.solve());
    Ok(())
}
