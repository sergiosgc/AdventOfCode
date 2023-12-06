use std::io::BufRead;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Race {
    time: i64,
    distance: i64
}
impl Race {
    pub fn ceil(x: f64) -> i64 {
        if x.trunc() == x { (x+1_f64).trunc() as i64 } else { x.ceil() as i64}
    }
    pub fn floor(x: f64) -> i64 {
        if x.trunc() == x { (x-1_f64).trunc() as i64 } else { x.floor() as i64}
    }
    pub fn record_interval(&self) -> Option<(i64, i64)> {
        let a: f64 = -1_f64;
        let b: f64 = self.time as f64;
        let c: f64 = (-self.distance) as f64;
        let determinant = (b.powi(2) - 4_f64*a*c).sqrt();
        if determinant < 0_f64 { None } else { 
            Some(
                ( Race::ceil((-b + determinant) / (2_f64 * a)), Race::floor((-b - determinant) / (2_f64 * a)) )
            )
        }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok);
    let races: Vec<Race> = input
    .next().unwrap()
    .split_once(":").unwrap().1
    .split_whitespace()
    .flat_map(str::parse::<i64>)
    .zip(
        input
        .next().unwrap()
        .split_once(":").unwrap().1
        .split_whitespace()
        .flat_map(str::parse::<i64>)
    )
    .map(|(time, distance)| Race { time, distance })
    .collect::<Vec<Race>>();


    println!("{:#?}", races.iter().flat_map(Race::record_interval).map(|(z1, z2)| z2 - z1 + 1).fold(1, |acc, v| acc * v));
    Ok(())
}
