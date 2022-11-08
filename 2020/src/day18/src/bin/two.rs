use std::io::BufRead;

use regex::Regex;
#[derive(Clone,Debug)]
struct Stack(Vec<StackCell>);
#[derive(Clone,Debug)]
struct StackCell {
    operator: Option<char>,
    value: Option<i64>
}
impl StackCell {
    fn execute(&self, stack: &mut Vec<i64>) -> () {
        match (self.operator, self.value) {
            (None, Some(v)) => { stack.push(v); },
            (Some('+'), None) => { 
                let mut val = stack.pop().unwrap();
                val += stack.pop().unwrap();
                stack.push( val );
            },
            (Some('*'), None) => { 
                let mut val = stack.pop().unwrap();
                val *= stack.pop().unwrap();
                stack.push( val );
            },
            _ => {}
        }
    }
}
impl Stack {
    fn new() -> Stack { Stack { 0: Vec::new() }}
    fn from_string(expr: String) -> Stack {
        let mut result = Stack::new();
        let mut operator_stack: Stack = Stack::new();
        let re = Regex::new(r"(?P<space> )|(?P<addition>[+])|(?P<multiplication>[*])|(?P<openparens>[(])|(?P<closeparens>[)])|(?P<number>[0-9]+)").unwrap();
        for capture in re.captures_iter(&expr) {
            match (capture.name("space"), capture.name("addition"), capture.name("multiplication"), capture.name("openparens"), capture.name("closeparens"), capture.name("number")) {
                (Some(_space), None, None, None, None, None) => {},
                (None, Some(operator), None, None, None, None) => {
                    while operator_stack.0.len() > 0 && operator_stack.0[operator_stack.0.len() - 1].operator.is_some() && operator_stack.0[operator_stack.0.len() - 1].operator.unwrap() != '(' && operator_stack.0[operator_stack.0.len() - 1].operator.unwrap() != '*' {
                        result.0.push(operator_stack.0.pop().unwrap());
                    }
                    operator_stack.0.push(StackCell { operator: Some(operator.as_str().chars().next().unwrap()), value: None });
                },
                (None, None, Some(operator), None, None, None) => {
                    while operator_stack.0.len() > 0 && operator_stack.0[operator_stack.0.len() - 1].operator.is_some() && operator_stack.0[operator_stack.0.len() - 1].operator.unwrap() != '(' {
                        result.0.push(operator_stack.0.pop().unwrap());
                    }
                    operator_stack.0.push(StackCell { operator: Some(operator.as_str().chars().next().unwrap()), value: None });
                },
                (None, None, None, Some(_openparens), None, None) => {
                    operator_stack.0.push(StackCell { operator: Some('('), value: None });
                },
                (None, None, None, None, Some(_closeparens), None) => {
                    while operator_stack.0.len() > 0 && operator_stack.0[operator_stack.0.len() - 1].operator.is_some() && operator_stack.0[operator_stack.0.len() - 1].operator.unwrap() != '(' {
                        result.0.push(operator_stack.0.pop().unwrap());
                    }
                    operator_stack.0.pop().unwrap();
                },
                (None, None, None, None, None, Some(number)) => {
                    result.0.push(StackCell { operator: None, value: Some(number.as_str().parse::<i64>().unwrap()) });
                },
                _ => {}
            }
        }
        while operator_stack.0.len() > 0 {
            result.0.push(operator_stack.0.pop().unwrap());
        }
        result
    }
    fn execute(&self) -> i64 {
        let mut stack: Vec<i64> = Vec::<i64>::new();
        for cell in &self.0 {
            cell.execute(&mut stack);
        }
        stack.pop().unwrap()
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<i64> = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).map(|l| Stack::from_string(l)).map(|s| s.execute()).collect();
    println!("{:#?}", input.into_iter().reduce(|a,b| a+b));
    Ok(())
}
