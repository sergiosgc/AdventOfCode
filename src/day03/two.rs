use std::io::BufRead;

pub fn matching(input: &Vec<String>, column: usize, o2: bool) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut linecount = 0;
    let mut ones = 0;
    let mut want = b'1';
    for line in input {
        linecount = linecount + 1;
        if line.as_bytes()[column] == b'1' { ones += 1; }
    }
    if ( ones < linecount - ones && o2 ) || ( ones >= linecount - ones && !o2 ) { want = b'0'; }
    for line in input {
        if line.as_bytes()[column] == want { result.push(String::from(line)); }
    }
    if result.len() == 1 { return result; }
    return matching(&result, column+1, o2);
}
pub fn binary_string_to_int(s: &String) -> i32 {
    let mut result: i32 = 0;
    for byte in s.bytes() {
        result = result << 1;
        if byte == b'1' { result += 1; }
    }
    return result;
}



pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    println!("{}", binary_string_to_int(&matching(&input, 0, true).as_slice()[0]) * binary_string_to_int(&matching(&input, 0, false).as_slice()[0]));
    return Ok(());
}

