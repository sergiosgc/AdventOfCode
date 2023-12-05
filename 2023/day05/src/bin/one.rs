use std::io::BufRead;
pub trait CategoryMapper<T> {
    fn map(&self, value: T) -> T;
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct CategoryMap {
    pub destination: i64,
    pub source: i64,
    pub length: i64
}
impl CategoryMap {
    pub fn from_str(s: &str) -> CategoryMap {
        let mut iter = s
        .split(" ").flat_map(str::parse::<i64>);
        CategoryMap { destination: iter.next().unwrap(), source: iter.next().unwrap(), length: iter.next().unwrap() }
    }
    pub fn maps(&self, value: i64) -> bool { value >= self.source && value < self.source + self.length }
}
impl CategoryMapper<i64> for CategoryMap {
    fn map(&self, value: i64) -> i64 {
        if !self.maps(value) { panic!("{:?} does not map {}", self, value); }
        self.destination - self.source + value
    }
}
impl CategoryMapper<i64> for Vec::<CategoryMap> {
    fn map(&self, value: i64) -> i64 {
        match self.iter().find(|category_map| category_map.maps(value)) {
            Some(category_map) => category_map.map(value),
            None => value
        }
    }
}
impl CategoryMapper<i64> for Vec::<Vec::<CategoryMap>> {
    fn map(&self, value: i64) -> i64 {
        self.iter().fold(value, |value, category_maps| {
            category_maps.map(value)
        })
    }
}
impl CategoryMapper<Vec::<i64>> for Vec::<CategoryMap> {
    fn map(&self, value: Vec::<i64>) -> Vec::<i64> {
        value.into_iter().map(|value| self.map(value) ).collect()
    }
}
impl CategoryMapper<Vec::<i64>> for Vec::<Vec::<CategoryMap>> {
    fn map(&self, value: Vec::<i64>) -> Vec::<i64> {
        value.into_iter().map(|value| self.map(value)).collect()
        
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok);
    let seeds = input
    .next()
    .unwrap()
    .split_once(":")
    .unwrap().1
    .split(" ")
    .flat_map(str::parse::<i64>)
    .collect::<Vec<i64>>();
    let category_maps = input
    .filter(|s| !s.is_empty())
    .fold(vec![] as Vec::<Vec::<CategoryMap>>, |mut acc: Vec::<Vec::<CategoryMap>>, line| -> Vec::<Vec::<CategoryMap>> {
        if line.contains("map") {
            acc.push(vec![]);
        } else {
            let last = acc.len()-1;
            acc[last].push(CategoryMap::from_str(&line));
        }
        acc
    });
    println!("{:#?}", category_maps.map(seeds).into_iter().min());
    Ok(())
}
