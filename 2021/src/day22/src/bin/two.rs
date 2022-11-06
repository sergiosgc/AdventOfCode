use std::io::BufRead;
use regex::Regex;
use std::cmp::{min, max};

#[derive(Clone,Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}
#[derive(Clone)]
struct Cuboid {
    left: Coord,
    right: Coord,
    light: i64
}
impl std::fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ({},{},{})x({},{},{})", 
            if self.light == 1 { "on" } else { "off" },
            self.left.x,
            self.left.y,
            self.left.z,
            self.right.x,
            self.right.y,
            self.right.z)
    }
}
impl Cuboid {
    fn new(on: bool, leftx: i64, rightx: i64, lefty: i64, righty: i64, leftz: i64, rightz: i64) -> Cuboid {
        Cuboid {
            light: if on { 1 } else { -1 },
            left: Coord { x: leftx, y: lefty, z: leftz },
            right: Coord { x: rightx, y: righty, z: rightz },
        }
    }
    fn parse(line: String) -> Cuboid {
        let captures = Regex::new(r"^(?P<on>on|off) x=(?P<xmin>[^.]*)..(?P<xmax>[^.]*),y=(?P<ymin>[^.]*)..(?P<ymax>[^.]*),z=(?P<zmin>[^.]*)..(?P<zmax>[^.]*)").unwrap().captures(&line).unwrap();
        Cuboid::new(
            captures.name("on").unwrap().as_str() == "on",
            captures.name("xmin").unwrap().as_str().parse::<i64>().unwrap(),
            captures.name("xmax").unwrap().as_str().parse::<i64>().unwrap(),
            captures.name("ymin").unwrap().as_str().parse::<i64>().unwrap(),
            captures.name("ymax").unwrap().as_str().parse::<i64>().unwrap(),
            captures.name("zmin").unwrap().as_str().parse::<i64>().unwrap(),
            captures.name("zmax").unwrap().as_str().parse::<i64>().unwrap(),
        )
    }
    fn intersect(&self, existing: Cuboid) -> Option<Cuboid> {
        let result = Cuboid {
            light: -1 * existing.light,
            left:  Coord { x: max(self.left.x, existing.left.x), y: max(self.left.y, existing.left.y), z: max(self.left.z, existing.left.z) },
            right: Coord { x: min(self.right.x, existing.right.x), y: min(self.right.y, existing.right.y), z: min(self.right.z, existing.right.z) },
        };
        if result.left.x > result.right.x || result.left.y > result.right.y || result.left.z > result.right.z { return None; }
        Some(result)
    }
    fn lit(&self) -> i64 { (self.right.x - self.left.x + 1) * (self.right.y - self.left.y + 1) * (self.right.z - self.left.z + 1) * self.light }
}
#[derive(Clone,Debug)]
struct Cuboids {
    members: Vec<Cuboid>
}
impl Cuboids {
    fn add(&mut self, cuboid: Cuboid) {
        let mut intersections: Vec<Cuboid> = self.members.clone()
            .into_iter()
            .map(|c| cuboid.intersect(c) )
            .filter(|c| c.is_some() )
            .map(|c| c.unwrap())
            .collect();
        self.members.append(&mut intersections);
        if cuboid.light == 1 { self.members.push(cuboid.clone()); }
    }
    fn from(cuboids: Vec<Cuboid>) -> Cuboids {
        let mut result = Cuboids { members: Vec::new() };
        cuboids.into_iter().for_each(|c| result.add(c) );
        result
    }
    fn lit(&self) -> i64 {
        self.members.clone().into_iter().map(|c| c.lit() ).reduce( |a,b| a + b ).unwrap()
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<Cuboid> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|l| Cuboid::parse(l))
        .collect();
    println!("{:?}", Cuboids::from(input).lit());
    Ok(())
}

