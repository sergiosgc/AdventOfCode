use std::{io::BufRead, collections::HashMap};
#[derive(Clone)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}
impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        (self.x-1..self.x+2).map(move |x| (self.y-1..self.y+2).map(move |y| (self.z-1..self.z+2).map(move |z| Coord{ x:x, y:y, z:z }) ).flatten() ).flatten()
            .filter(|c| c.x != self.x || c.y != self.y || c.z != self.z)
            .collect()
    }
}
impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Coord({},{},{})", self.x, self.y, self.z) }
}
impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
#[derive(Clone)]
struct Cubes {
    active: HashMap<i32, HashMap<i32, HashMap<i32, bool>>>,
}
impl Cubes {
    fn get(&self, c: Coord) -> bool {
        match self.active.get(&c.x) {
            None => false,
            Some(activex) => match activex.get(&c.y) {
                None => false,
                Some(activey) => match activey.get(&c.z) {
                    None => false,
                    Some(activez) => *activez
                }
            }
        }
    }
    fn set(&mut self, c: Coord) -> () {
        match self.active.get_mut(&c.x) {
            None => { self.active.insert(c.x, HashMap::new()); }
            Some(_activex) => {}
        };
        match self.active.get_mut(&c.x).unwrap().get_mut(&c.y) {
            None => { self.active.get_mut(&c.x).unwrap().insert(c.y, HashMap::new()); }
            Some(_activex) => {}
        };
        match self.active.get_mut(&c.x).unwrap().get_mut(&c.y).unwrap().get_mut(&c.z) {
            None => { self.active.get_mut(&c.x).unwrap().get_mut(&c.y).unwrap().insert(c.z, true); }
            Some(_activex) => {}
        };
    }
    fn unset(&mut self, c: Coord) -> () {
        match self.active.get_mut(&c.x) {
            None => { return; }
            Some(activex) => match activex.get_mut(&c.y) {
                None => { return; }
                Some(activey) => match activey.get_mut(&c.z) {
                    None => { return; }
                    Some(_activez) => {
                        activey.remove(&c.z);
                    }
                }
            }
        };
        if self.active.get(&c.x).unwrap().get(&c.y).unwrap().len() == 0 { self.active.get_mut(&c.x).unwrap().remove(&c.y); }
        if self.active.get(&c.x).unwrap().len() == 0 { self.active.remove(&c.x); }
    }
    fn set_value(&mut self, c: Coord, value: bool) -> () { return if value { self.set(c); } else { self.unset(c); }; }
    fn from_strings(strings: Vec<String>) -> Cubes {
        let mut result = Cubes { active: HashMap::new() };
        for (y, line) in strings.into_iter().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr != '#' { continue }
                result.set(Coord { x:x as i32, y:y as i32, z:0 });
            }
        }
        result
    }
    fn left_coord(&self) -> Coord {
        let mut result = Coord{ x:0, y:0, z:0 };
        for (x,xs) in self.active.iter() {
            result.x = result.x.min(*x);
            for (y,ys) in xs.iter() {
                result.y = result.y.min(*y);
                for (z,_zs) in ys.iter() {
                    result.z = result.z.min(*z);
                }
            }
        }
        result
    }
    fn right_coord(&self) -> Coord {
        let mut result = Coord{ x:0, y:0, z:0 };
        for (x,xs) in self.active.iter() {
            result.x = result.x.max(*x);
            for (y,ys) in xs.iter() {
                result.y = result.y.max(*y);
                for (z,_zs) in ys.iter() {
                    result.z = result.z.max(*z);
                }
            }
        }
        result
    }
    fn cycle_changes(&self) -> Vec<(bool, Coord)> {
        let left = self.left_coord() + Coord{ x:-1,y:-1,z:-1 };
        let right = self.right_coord() + Coord{ x:1,y:1,z:1 };
        (left.x..right.x+1).map(move |x| 
            (left.y..right.y+1).map(move |y| 
                (left.z..right.z+1).map(move |z| {
                    match (self.get(Coord { x:x, y:y, z:z }), (Coord { x:x,y:y,z:z}).neighbours().into_iter().filter(|n| self.get(n.clone()) ).collect::<Vec<Coord>>().len()) {
                        (true, 2) | (true, 3) => (true, Coord{ x:x, y:y, z:z }),
                        (false, 3) => (true, Coord { x:x, y:y, z:z }),
                        _ => (false, Coord { x:x, y:y, z:z })
                    }
                })
            ).flatten()
        ).flatten()
        .collect::<Vec<(bool, Coord)>>()
    }
    fn cycle(&mut self) -> () {
        self.cycle_changes()
            .into_iter()
            .for_each(|(val, coord)| self.set_value(coord, val) )
    }
    fn active(&self) -> Vec<Coord> {
        let left = self.left_coord();
        let right = self.right_coord();
        (left.x..right.x+1).map(move |x| 
            (left.y..right.y+1).map(move |y| 
                (left.z..right.z+1).map(move |z| {
                    if self.get(Coord { x:x, y:y, z:z }) { Some(Coord{ x:x, y:y, z:z }) } else { None }
                })
            ).flatten()
        ).flatten()
        .filter(|c| c.is_some() )
        .map( |c| c.unwrap() )
        .collect()
    }
}
impl std::fmt::Debug for Cubes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = self.left_coord();
        let right = self.right_coord();
        for z in left.z..right.z+1 {
            writeln!(f, "z={}", z).unwrap();
            for y in left.y..right.y+1 {
                for x in left.x..right.x+1 {
                    write!(f, "{}", if self.get( Coord { x:x, y:y, z:z }) { "#" } else { "." }).unwrap();
                }
                writeln!(f, "").unwrap();
            }
            writeln!(f, "").unwrap();
        }
        writeln!(f, " ")
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Cubes = Cubes::from_strings(std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect());
    (0..6).for_each(|_i| input.cycle() );
    println!("{:#?}", input.active().len());
    Ok(())
}
