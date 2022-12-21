use regex::RegexSet;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub enum Operation {
    Addition(Vec<String>),
    Subtraction(Vec<String>),
    Multiplication(Vec<String>),
    Division(Vec<String>),
}
#[derive(Clone, Debug)]
pub struct Term {
    pub coefficient: i64,
}
#[derive(Clone, Debug)]
pub enum Expression {
    Operation(Operation),
    Term(Term),
    Unknown
}
impl Expression {
    pub fn from_string(expression: &str) -> Expression {
        let set = RegexSet::new([
            r"\d+",
            r"[^ ]+ [-+*/] [^ ]+",
            r"^x",
        ]).unwrap();
        let set_matches = set.matches(expression);
        match (set_matches.matched(0), set_matches.matched(1), set_matches.matched(2)) {
            (true, false, false) => Expression::Term(Term { coefficient: expression.parse::<i64>().unwrap() }),
            (false, true, false) => {
                let (left, operation, right) = expression.split(" ").next_tuple().unwrap();
                let operands = vec![left.to_string(), right.to_string()];
                Expression::Operation(match operation {
                    "+" => Operation::Addition(operands),
                    "-" => Operation::Subtraction(operands),
                    "*" => Operation::Multiplication(operands),
                    "/" => Operation::Division(operands),
                    _ => panic!("Unexpected operation: '{}'", operation),
                })
            },
            (false, false, true) => Expression::Unknown,
            _ => panic!("No matches for {}", expression)
        }
    }
}