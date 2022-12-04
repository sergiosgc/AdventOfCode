use std::io::BufRead;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:#?}", std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok)
        .map(|line| line
            .replace("-", ",")
            .split(",")
            .map( |v| v.parse::<i64>().unwrap() )
            .collect::<Vec<i64>>()
        ).map(|line| [
            (line[0], line[1]), 
            (line[2], line[3]), 
            (line[0].max(line[2]), line[1].min(line[3]))
        ]).filter(|ranges| ranges[2].0 <= ranges[2].1)
        .count()
    );
    Ok(())
}