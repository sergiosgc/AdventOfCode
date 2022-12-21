use std::collections::HashMap;
use itertools::Itertools;
use crate::expression::{Expression, Operation};

#[derive(Clone, Debug)]
pub struct Calculator {
    expressions: HashMap<String, Expression>,
}
impl Calculator {
    pub fn from_strings(lines: Vec<String>) -> Calculator {
        Calculator {
            expressions: lines.iter()
                .map(|line| line.split(": ").next_tuple().unwrap())
                .fold(HashMap::<String, Expression>::new(), |mut expressions: HashMap<String, Expression>, (key, expression): (&str, &str)| {
                    expressions.insert(key.to_string(), Expression::from_string(expression));
                    expressions
                }),
        }
    }
    pub fn calculate(&self, expression: &String, unknown: Option<i64>) -> i64 {
        match self.expressions.get(expression).unwrap() {
            Expression::Unknown => unknown.unwrap(),
            Expression::Term(term) => term.coefficient,
            Expression::Operation(operation) => match operation {
                Operation::Addition(operands) => self.calculate(&operands[0], unknown)       + self.calculate(&operands[1], unknown),
                Operation::Subtraction(operands) => self.calculate(&operands[0], unknown)    - self.calculate(&operands[1], unknown),
                Operation::Multiplication(operands) => self.calculate(&operands[0], unknown) * self.calculate(&operands[1], unknown),
                Operation::Division(operands) => self.calculate(&operands[0], unknown)       / self.calculate(&operands[1], unknown),
            }
        }
    }
}