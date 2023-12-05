use std::io::BufRead;
pub trait CategoryMapper {
    fn map(&self, value: ValueRange) -> (Vec::<ValueRange>, Vec::<ValueRange>);
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValueRange {
    pub start: i64,
    pub end: i64
}
impl ValueRange {
    pub fn from_vec(v: &mut Vec::<i64>) -> Vec<ValueRange> {
        let mut result: Vec<ValueRange> = Vec::<ValueRange>::new();
        while !v.is_empty() {
            result.push(ValueRange { start: v[0], end: v[0] + v[1] - 1 });
            *v = v.split_off(2);
        }
        result
    }
    pub fn left_common_right(self, other: ValueRange) -> (Option<ValueRange>, Option<ValueRange>, Option<ValueRange>) {
        (
            if other.start < self.start {
                Some(ValueRange { start: other.start, end: other.end.min(self.start - 1) } )
            } else {
                None
            },
            if other.start <= self.end && other.end >= self.start {
                Some( ValueRange { start: other.start.max(self.start), end: other.end.min(self.end) })
            } else {
                None
            },
            if other.end > self.end {
                Some( ValueRange { start: other.start.max(self.end + 1), end: other.end })
            } else {
                None
            }
        )
    }
    pub fn minus(self, other: ValueRange) -> Option<ValueRange> {
        if other.start <= self.start && other.end >= self.end { None }
        else if other.end < self.start || other.start > self.end { Some(self) }
        else { Some(ValueRange { 
            start: if other.start <= self.start { other.end + 1 } else { self.start },
            end: if other.start <= self.start { self.end } else { other.start - 1 },
        }) }
    }
}
pub fn auto_subtract(mut values: Vec::<ValueRange>) -> Vec::<ValueRange> {
    if values.len() == 0 {
        vec![]
    } else {
        let tail = values.split_off(1);
        let subtracted = tail.iter().fold(Some(values[0]), |left, right| if let Some(left) = left { left.minus(*right) } else { None });
        let mut result = if let Some(subtracted) = subtracted { vec![subtracted] } else { vec![] };
        result.append(&mut auto_subtract(tail));
        result
    }


}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct CategoryMap {
    pub destination: i64,
    pub source: ValueRange
}
impl CategoryMap {
    pub fn from_str(s: &str) -> CategoryMap {
        let mut iter = s
        .split(" ").flat_map(str::parse::<i64>);
        let destination = iter.next().unwrap();
        let source = iter.next().unwrap();
        let range = iter.next().unwrap();
        CategoryMap { destination: destination, source: ValueRange { start: source, end: source + range - 1 } }
    }
    pub fn maps(&self, value: i64) -> bool { value >= self.source.start && value <= self.source.end }
}
impl CategoryMapper for CategoryMap {
    fn map(&self, value: ValueRange) -> (Vec::<ValueRange>, Vec::<ValueRange>) {
        let mut unmapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        let mut mapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        let (left, common, right) = self.source.left_common_right(value);
        if let Some(common) = common { 
            mapped.push(ValueRange { start: self.destination - self.source.start + common.start, end: self.destination - self.source.start + common.end });
            if let Some(left) = left { unmapped.push(left); }
            if let Some(right) = right { unmapped.push(right); }
            (unmapped, mapped)
        } else {
            (vec![value], vec![])
        }
    }
}
impl CategoryMapper for Vec<CategoryMap> {
    fn map(&self, value: ValueRange) -> (Vec::<ValueRange>, Vec::<ValueRange>) {
        let mut unmapped: Vec<ValueRange> = vec![value];
        let mut mapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        for category_map in self {
            let (just_unmapped, mut just_mapped) = unmapped
            .iter()
            .map(|value_range| category_map.map(*value_range))
            .fold((Vec::<ValueRange>::new(), Vec::<ValueRange>::new()), |(mut unmapped_acc, mut mapped_acc), (unmapped, mapped)| {
                unmapped_acc.append( &mut unmapped.clone() );
                mapped_acc.append( &mut mapped.clone() );
                (unmapped, mapped)
            });
            unmapped = auto_subtract(just_unmapped);
            mapped.append( &mut just_mapped );
            mapped = auto_subtract(mapped);
        }
        (unmapped, mapped)
    }
}
impl CategoryMapper for Vec<Vec<CategoryMap>> {
    fn map(&self, value: ValueRange) -> (Vec::<ValueRange>, Vec::<ValueRange>) {
        let mut unmapped: Vec<ValueRange> = vec![value];
        let mut mapped: Vec<ValueRange> = Vec::<ValueRange>::new();
        unmapped.push(value);
        for category_map in self {
            let (mut just_unmapped, mut just_mapped) = unmapped
            .iter()
            .map(|value_range| category_map.map(*value_range))
            .fold((Vec::<ValueRange>::new(), Vec::<ValueRange>::new()), |(mut unmapped_acc, mut mapped_acc), (unmapped, mapped)| {
                unmapped_acc.append( &mut unmapped.clone() );
                mapped_acc.append( &mut mapped.clone() );
                (unmapped_acc, mapped_acc)
            });
            unmapped.clear();
            unmapped.append( &mut just_mapped.clone() );
            unmapped.append( &mut just_unmapped.clone() );
            unmapped = auto_subtract(unmapped);
        }

        (vec![], unmapped)
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok);
    let seeds = ValueRange::from_vec(&mut input
    .next()
    .unwrap()
    .split_once(":")
    .unwrap().1
    .split(" ")
    .flat_map(str::parse::<i64>)
    .collect::<Vec<i64>>());

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
    println!("{:#?}", seeds.into_iter().flat_map(|seed| category_maps.map(seed).1).map(|range| range.start).min());
    Ok(())
}
