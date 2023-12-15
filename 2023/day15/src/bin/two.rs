use std::io::BufRead;
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Operation {
    Remove(String),
    Add(String,i64)
}
pub fn hash(s: &str) -> usize {
    s.chars().fold(0_usize, |acc, ch| ((ch as i64 + acc as i64) * 17 % 256).try_into().unwrap())
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:#?}", std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .next().unwrap()
        .split(",")
        .map(|op: &str| if op.find("=").is_some() {
                let mut i = op.split("=");
                Operation::Add( i.next().unwrap().to_string(), i.next().unwrap().parse::<i64>().unwrap())
            } else {
                Operation::Remove(op.replace("-", ""))
            }
        )
        .fold(vec![Vec::<(String, i64)>::new(); 256], |mut boxes, op| {
            match op {
                Operation::Remove(label) => {
                    if let Some(index) = boxes[hash(&label)].iter().position(|lens| lens.0 == label) {
                        boxes[hash(&label)].remove(index);
                    }
                }
                Operation::Add(label, focal_length) => {
                    match boxes[hash(&label)].iter().position(|lens| lens.0 == label) {
                        Some(index) => { boxes[hash(&label)][index].1 = focal_length; },
                        None => { boxes[hash(&label)].push((label, focal_length)); }
                    }
                }
            }
            boxes
        })
        .iter().enumerate().flat_map(|(r#box, lenses)|
            lenses.iter().enumerate().map(move |(slot, lens)| (1_i64+r#box as i64) * (1_i64+slot as i64) * lens.1)
        )
        .sum::<i64>()
    );
    Ok(())
}

