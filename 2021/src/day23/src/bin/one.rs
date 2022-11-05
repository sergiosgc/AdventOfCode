use std::io::BufRead;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone,Debug)]
struct BurrowState {
    energy: i64,
    burrow: [String; 5]
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
            ]
        }
    }
    fn solve(&self) -> BurrowState {
        let mut stack: Vec<BurrowState> = Vec::new();
        let mut visited: HashSet<BurrowState> = HashSet::new();
        stack.push(self.clone());
        loop {
            let mut current = stack.pop().unwrap();
            while match visited.get(&current) { Some(_b) => true, None => false } { current = stack.pop().unwrap(); }
            visited.insert(current.clone());
            println!("{:#?}", current);
            if  &current.burrow[2][3..4] == "A" &&
                &current.burrow[3][3..4] == "A" &&
                &current.burrow[2][5..6] == "B" &&
                &current.burrow[3][5..6] == "B" &&
                &current.burrow[2][7..8] == "C" &&
                &current.burrow[3][7..8] == "C" &&
                &current.burrow[2][9..10] == "D" &&
                &current.burrow[3][9..10] == "D" { return current; }
            let mut next: Vec<BurrowState> = current.gen_moves()
                .into_iter()
                .filter( |b| match visited.get(b) { Some(_b) => false, None => true })
                .collect();
            stack.append( &mut next );
            stack.sort_unstable_by(|a, b| b.energy.cmp(&a.energy));
        }
    }
    fn gen_moves(&self) -> Vec<BurrowState> {
        let mut result: Vec<BurrowState> = Vec::new();
        for y in 0..5 {
            for x in 0..12 {
                for delta in [(-1,0), (1,0), (0,1), (0, -1)] {
                    result.append(&mut self.move_amphipod((x,y), delta, true).into_iter().map( |x| x.1 ).collect())
                }
            }
        }
        result
    }
    fn move_amphipod(&self, which: (usize, usize), delta: (i32, i32), test_hallway_start: bool) -> Vec<((i32, i32), BurrowState)> {
        let mut result: Vec<((i32, i32), BurrowState)> = Vec::new();
        if self.burrow[which.1][which.0..which.0+1] != "A".to_string() && // Only move amphipods (ABCD) to empty space (.)
           self.burrow[which.1][which.0..which.0+1] != "B".to_string() && 
           self.burrow[which.1][which.0..which.0+1] != "C".to_string() && 
           self.burrow[which.1][which.0..which.0+1] != "D".to_string() ||
           self.burrow[((which.1 as i32) + delta.1) as usize][((which.0 as i32) + delta.0) as usize..((which.0 as i32) + delta.0 + 1) as usize] != ".".to_string() { return result; }
        if delta == (0,1) && which.1 == 1 { // Only move down on the correct room
            if which.0 != match &self.burrow[which.1][which.0..which.0+1] {
                "A" => 3,
                "B" => 5,
                "C" => 7,
                "D" => 9,
                _ => panic!("Unexpected amphipod")
            } { return result; }
            if &self.burrow[which.1 + 2][which.0..which.0+1] != "." && self.burrow[which.1 + 2][which.0..which.0+1] != self.burrow[which.1][which.0..which.0+1] { return result; } // Don't move down if not final destination yet (room occupied by wrong amphipod)
            let maybe = self.move_amphipod(which, (0,2), test_hallway_start); // Try moving all the way down
            if maybe.len() > 0 { return maybe; }
        }
        if delta == (0,-1) && which.1 == 3 {
            let maybe = self.move_amphipod(which, (0,-2), test_hallway_start); // Try moving all the way up
            if maybe.len() > 0 { return maybe; }
        }
        let next = self._move_amphipod(which, ((which.0 as i32 + delta.0) as usize, (which.1 as i32 + delta.1) as usize));
        result.push(((0,0), next.clone()));
        match delta {
            (1,0) | (-1, 0) => {
                result.append( &mut next.move_amphipod( ((which.0 as i32 + delta.0) as usize, (which.1 as i32 + delta.1) as usize), (0,1), false ));
                result.append( &mut next.move_amphipod( ((which.0 as i32 + delta.0) as usize, (which.1 as i32 + delta.1) as usize), delta, false ));
            },
            (0,-1) | (0,-2) => {
                result.append( &mut next.move_amphipod( ((which.0 as i32 + delta.0) as usize, (which.1 as i32 + delta.1) as usize), (1,0), false ));
                result.append( &mut next.move_amphipod( ((which.0 as i32 + delta.0) as usize, (which.1 as i32 + delta.1) as usize), (-1,0), false ));
                result.append( &mut next.move_amphipod( ((which.0 as i32 + delta.0) as usize, (which.1 as i32 + delta.1) as usize), (0,-1), false ));
            }
            (0,1) | (0,2) => {
                result.append( &mut next.move_amphipod( ((which.0 as i32 + delta.0) as usize, (which.1 as i32 + delta.1) as usize), (0,1), false ));
            },
            _ => panic!("Unexpected move_amphipod delta {:?}", delta)
        }
        result.into_iter()
            .filter( |state| 
                state.1.burrow[1][3..4] == ".".to_string() &&
                state.1.burrow[1][5..6] == ".".to_string() &&
                state.1.burrow[1][7..8] == ".".to_string() &&
                state.1.burrow[1][9..10] == ".".to_string())
            .filter( |state| !(
                state.1.burrow[2][3..4] != ".".to_string() && state.1.burrow[3][3..4] == ".".to_string() ||
                state.1.burrow[2][5..6] != ".".to_string() && state.1.burrow[3][5..6] == ".".to_string() ||
                state.1.burrow[2][7..8] != ".".to_string() && state.1.burrow[3][7..8] == ".".to_string() ||
                state.1.burrow[2][9..10] != ".".to_string() && state.1.burrow[3][9..10] == ".".to_string()))
            .map( |state| ((state.0.0 + delta.0, state.0.1 + delta.1), state.1))
            .filter( |state| !test_hallway_start || which.1 != 1 || state.0.1 != 0 )
            .collect()
    }
    fn _move_amphipod(&self, from: (usize, usize), to: (usize, usize)) -> BurrowState {
        let energy = match &self.burrow[from.1][from.0..from.0+1] {
            "A" => 1,
            "B" => 10,
            "C" => 100,
            "D" => 1000,
            _ => panic!("Unexpected target to move_amphipod")
        };
        let mut result = self.clone();
        result.energy += energy * (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as i64;
        result.burrow[to.1].replace_range(to.0..to.0+1, &result.burrow[from.1][from.0..from.0+1].to_owned());
        result.burrow[from.1].replace_range(from.0..from.0+1, ".");
        result
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: BurrowState = BurrowState::parse(std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect());
//    println!("{:#?}", input);
//    println!("{:#?}", input._move_amphipod((7, 2), (4, 1)));
    //println!("{:#?}", input.move_amphipod((7, 2), (0, -1), false)[2].1.move_amphipod((11,1), (-1,0), true));
    //println!("{:#?}", input.solve());
    //println!("{:#?}", input.move_amphipod((1, 1), (1, 0), false));
    println!("{:#?}", input.solve());
    Ok(())
}
