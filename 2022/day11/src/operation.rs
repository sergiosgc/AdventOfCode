use regex::Regex;

#[derive(Debug,Clone,Copy)]
pub enum Operand {
    Value(i64),
    Old
}
impl Operand {
    pub fn from_string(s: &str) -> Operand {
        match s {
            "old" => Operand::Old,
            v @ _ => Operand::Value(v.parse::<i64>().unwrap())
        }
    }
    pub fn value(self, old: i64) -> i64 {
        match self {
            Operand::Old => old,
            Operand::Value(v) => v
        }
    }
}
#[derive(Debug,Clone,Copy)]
pub enum Operator {
    Add,
    Multiply
}

#[derive(Debug,Clone,Copy)]
pub struct Operation {
    pub left_operand: Operand,
    pub right_operand: Operand,
    pub operator: Operator
}
impl Operation {
    pub fn from_string(s: &String) -> Operation {
        let captures = Regex::new(r"new = (?P<left_operand>old|\d+) (?P<operator>[+*]) (?P<right_operand>old|\d+)")
            .unwrap()
            .captures(s)
            .unwrap();
        Operation {
            left_operand: Operand::from_string(captures.name("left_operand").unwrap().as_str()),
            right_operand: Operand::from_string(captures.name("right_operand").unwrap().as_str()),
            operator: match captures.name("operator").unwrap().as_str() {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                _ => panic!("Unexpected operator")
            }
        }
    }
    pub fn execute(self, old: i64) -> i64 {
        match self.operator {
            Operator::Add => self.left_operand.value(old) + self.right_operand.value(old),
            Operator::Multiply => self.left_operand.value(old) * self.right_operand.value(old),
        }
    }
}