use std::cmp::Ordering;
#[derive(Clone, Debug)]
pub struct Recipe {
    pub index: i64,
    pub robot_cost_ore: i64,
    pub robot_cost_clay: i64,
    pub robot_cost_obsidian: (i64, i64),
    pub robot_cost_geode: (i64, i64),
}
impl Recipe {
    pub fn from_tuple(tuple: (i64, i64,i64,i64,i64,i64,i64)) -> Recipe {
        Recipe {
            index: tuple.0,
            robot_cost_ore: tuple.1,
            robot_cost_clay: tuple.2,
            robot_cost_obsidian: (tuple.3, tuple.4),
            robot_cost_geode: (tuple.5, tuple.6),
        }
    }
    pub fn quality(self, depth: i64) -> i64 {
        let mut stack = vec![SearchState { depth, ..Default::default() }];
        let mut result: Vec<SearchState> = Vec::new();
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            let next = current.successors(&self);
            let (mut goal, mut non_goal): (Vec<SearchState>, Vec<SearchState>) = next.into_iter().partition(|state| state.minute == depth);
            result.append(&mut goal);
            if result.len() > 1 {
                result.sort();
                result.reverse();
                result.resize(1, Default::default());
            }
            stack.append(&mut non_goal);
            stack.sort();
        }
        result.sort();
        let max = result.pop().unwrap();
        self.index * max.geode
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SearchState {
    pub ore: i64,
    pub clay: i64,
    pub obsidian: i64,
    pub geode: i64,
    pub robots_ore: i64,
    pub robots_clay: i64,
    pub robots_obsidian: i64,
    pub robots_geode: i64,
    pub minute: i64,
    pub depth: i64,
}
impl Default for SearchState {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            robots_ore: 1,
            robots_clay: 0,
            robots_obsidian: 0,
            robots_geode: 0,
            minute: 0,
            depth: 0,
        }
    }
}
impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic().cmp(&other.heuristic())
    }
}

impl SearchState {
    pub fn successors(self, recipe: &Recipe) -> Vec<SearchState> {
        let mut result: Vec<SearchState> = Vec::new();
        result.push({
            let time_needed = self.minutes_left();
            let mut next = self.clone();
            next.minute += time_needed;
            next.ore += next.robots_ore * time_needed;
            next.clay += next.robots_clay * time_needed;
            next.obsidian += next.robots_obsidian * time_needed;
            next.geode += next.robots_geode * time_needed;

            next
        });
        if self.robots_obsidian > 0 {
            result.push({
                let time_needed = (((recipe.robot_cost_geode.0 - self.ore).max(0) as f64 / self.robots_ore as f64).ceil() as i64)
                .max(((recipe.robot_cost_geode.1 - self.obsidian).max(0) as f64 / self.robots_obsidian as f64).ceil() as i64) 
                 + 1;
                let mut next = self.clone();
                next.minute += time_needed;
                next.ore += next.robots_ore * time_needed;
                next.clay += next.robots_clay * time_needed;
                next.obsidian += next.robots_obsidian * time_needed;
                next.geode += next.robots_geode * time_needed;
                
                next.robots_geode += 1;
                next.ore -= recipe.robot_cost_geode.0;
                next.obsidian -= recipe.robot_cost_geode.1;
                
                next
            });
        }
        if self.robots_clay > 0 {
            result.push({
                let time_needed = (((recipe.robot_cost_obsidian.0 - self.ore).max(0) as f64 / self.robots_ore as f64).ceil() as i64)
                .max(((recipe.robot_cost_obsidian.1 - self.clay).max(0) as f64 / self.robots_clay as f64).ceil() as i64) 
                 + 1;
                let mut next = self.clone();
                next.minute += time_needed;
                next.ore += next.robots_ore * time_needed;
                next.clay += next.robots_clay * time_needed;
                next.obsidian += next.robots_obsidian * time_needed;
                next.geode += next.robots_geode * time_needed;
                
                next.robots_obsidian += 1;
                next.ore -= recipe.robot_cost_obsidian.0;
                next.clay -= recipe.robot_cost_obsidian.1;
                
                next
            });
        }
        result.push({
            let time_needed = (((recipe.robot_cost_clay - self.ore).max(0) as f64 / self.robots_ore as f64).ceil() as i64) + 1;
            let mut next = self.clone();
            next.minute += time_needed;
            next.ore += next.robots_ore * time_needed;
            next.clay += next.robots_clay * time_needed;
            next.obsidian += next.robots_obsidian * time_needed;
            next.geode += next.robots_geode * time_needed;
            
            next.robots_clay += 1;
            next.ore -= recipe.robot_cost_clay;
            
            next
        });
        result.push({
            let time_needed = (((recipe.robot_cost_ore - self.ore).max(0) as f64 / self.robots_ore as f64).ceil() as i64) + 1;
            let mut next = self.clone();
            next.minute += time_needed;
            next.ore += next.robots_ore * time_needed;
            next.clay += next.robots_clay * time_needed;
            next.obsidian += next.robots_obsidian * time_needed;
            next.geode += next.robots_geode * time_needed;

            next.robots_ore += 1;
            next.ore -= recipe.robot_cost_ore;
            
            next
        });
        result.into_iter()
            .filter(|state| state.minute <= self.depth)
            .filter(|state| state.robots_ore <= recipe.robot_cost_ore.max(recipe.robot_cost_clay.max(recipe.robot_cost_obsidian.0.max(recipe.robot_cost_geode.0))))
            .filter(|state| state.robots_clay <= recipe.robot_cost_obsidian.1)
            .collect::<Vec<SearchState>>()

    }

    fn minutes_left(&self) -> i64 {
        self.depth - self.minute
    }
    pub fn heuristic(&self) -> i64 {
        let ore = self.ore + self.minutes_left() * self.robots_ore;
        let clay = self.clay + self.minutes_left() * self.robots_clay;
        let obsidian = self.obsidian + self.minutes_left() * self.robots_obsidian;
        let geode = self.geode + self.minutes_left() * self.robots_geode;
        geode * (1000_i64.pow(4)) + obsidian * (1000_i64.pow(3)) + clay * (1000_i64.pow(2)) + ore * 1000_i64 + self.minute
    }
}