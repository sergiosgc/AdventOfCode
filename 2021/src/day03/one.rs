use std::io::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut linecount = 0;
    let mut ones: [i32; 12] = [0; 12];
    for line in std::io::stdin().lock().lines() {
        linecount += 1;
        let mut index = 0;
        for b in line?.as_bytes() { 
            if *b == b'1' { ones[index] += 1; } 
            index = index + 1;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for one in ones {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if one > linecount - one {
            gamma = gamma | 1;
        } else {
            epsilon = epsilon | 1;
        }
    }

    println!("{}", gamma * epsilon);
    return Ok(());
}
