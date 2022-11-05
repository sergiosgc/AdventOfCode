use std::io::BufRead;
use regex::Regex;
#[derive(Clone,Debug)]
struct Input {
    x: (i64, i64),
    y: (i64, i64)
}
#[derive(Clone,Debug)]
struct Coord {
    x: i64,
    y: i64,
}
#[derive(Clone,Debug)]
struct Hit {
    v: Coord,
    maxy: i64
}
impl Input {
    fn parse(input: Vec<String>) -> Input {
        let re = Regex::new(r"^target area: x=(?P<xmin>[^.]*)..(?P<xmax>[^.]*), y=(?P<ymin>[^.]*)..(?P<ymax>[^.]*)").unwrap();
        let captures = re.captures(&input[0]).unwrap();
        Input {
            x: (captures.name("xmin").unwrap().as_str().parse::<i64>().unwrap(), captures.name("xmax").unwrap().as_str().parse::<i64>().unwrap()),
            y: (captures.name("ymin").unwrap().as_str().parse::<i64>().unwrap(), captures.name("ymax").unwrap().as_str().parse::<i64>().unwrap()),
        }
    }
    fn between(min: i64, value: i64, max: i64) -> bool { min <= value && value <= max }
    fn hits(&self) -> Vec<Hit> {
        let mut result: Vec<Hit> = Vec::new();
        for vx_zero in 0..500 as i64 {
            for vy_zero in -500..500 as i64 {
                let mut pos: Coord = Coord { x: 0, y: 0};
                let mut v: Coord = Coord { x: vx_zero, y: vy_zero };
                let mut maxy: i64 = 0;
                let mut its_a_hit = false;
                for _iter in 0..500 as i64 {
                    if Input::between(self.x.0, pos.x, self.x.1) && Input::between(self.y.0, pos.y, self.y.1) { its_a_hit = true; }
                    pos.x += v.x;
                    pos.y += v.y;
                    maxy = std::cmp::max(maxy, pos.y);
                    v.x -= if v.x == 0 { 0 } else { v.x / v.x.abs() };
                    v.y -= 1;
                }
                if its_a_hit {
                    result.push( Hit {
                        v: Coord { x: vx_zero, y: vy_zero },
                        maxy: maxy.clone()
                    });
                }
            }
        }
        result

    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Input = Input::parse(std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect());
    println!("{:#?}", input.hits().len());
    Ok(())
}