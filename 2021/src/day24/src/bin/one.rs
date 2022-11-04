use std::io::BufRead;
use regex::Regex;
#[derive(Clone,Debug)]
struct Program {
    div: i64,
    chk: i64,
    add: i64
}
impl Program {
    fn parse(input: Vec<String>) -> Vec<Program> {
        let re = Regex::new(r"(?P<operation>[^ ]*) (?P<operand1>[^ ]*)(?: (?P<operand2>[^ ]*))?").unwrap();
        (0..14).map(|i:i64| -> Program {
            Program {
                div: re.captures(&input[(i*18 + 4) as usize]).unwrap().name("operand2").unwrap().as_str().parse::<i64>().unwrap(),
                chk: re.captures(&input[(i*18 + 5) as usize]).unwrap().name("operand2").unwrap().as_str().parse::<i64>().unwrap(),
                add: re.captures(&input[(i*18 + 15) as usize]).unwrap().name("operand2").unwrap().as_str().parse::<i64>().unwrap(),
            }
        }).collect::<Vec<Program>>()
    }
    fn solve(programs: Vec<Program>, mut input: [i64; 14]) -> i64 {
        let mut stack: Vec<(usize, i64)> = Vec::new();
        let mut j: usize;
        let mut add: i64;
        for (i, program) in programs.iter().enumerate() {
            if program.div == 1 {
                stack.push( (i, program.add) );
            } else {
                (j, add) = stack.pop().unwrap();
                input[i] = input[j] + add + program.chk;
                if input[i] > 9 {
                    input[j] -= input[i] - 9;
                    input[i] = 9;
                }
                if input[i] < 1 {
                    input[j] += 1 - input[i];
                    input[i] = 1;
                }
            }
        }
        input.into_iter().reduce(|acc, v| acc * 10 + v).unwrap()
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<Program> = Program::parse(std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect());
    println!("Part 1: {:?}", Program::solve(input, [9,9,9,9,9,9,9,9,9,9,9,9,9,9]));
    Ok(())
}
