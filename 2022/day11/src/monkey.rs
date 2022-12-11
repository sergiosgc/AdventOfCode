use regex::Regex;

use crate::operation::Operation;

#[derive(Clone, Debug)]
pub struct Monkey {
    pub items: Vec<i64>,
    pub new_expression: Operation,
    pub test_divisor: i64,
    pub true_target: i64,
    pub false_target: i64,
    pub throw_count: i64,
}
impl Monkey {
    pub fn from_string(s: &[String]) -> Monkey {
        Monkey {
            items: {
                s[1].replace(" ", "")
                    .split(":")
                    .last()
                    .unwrap()
                    .split(",")
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            },
            new_expression: Operation::from_string(&s[2]),
            test_divisor: Regex::new(r"divisible by (\d+)")
                .unwrap()
                .captures(&s[3])
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
            true_target: Regex::new(r"If true: throw to monkey (\d+)")
                .unwrap()
                .captures(&s[4])
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
            false_target: Regex::new(r"If false: throw to monkey (\d+)")
                .unwrap()
                .captures(&s[5])
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
            throw_count: 0
        }
    }
}
pub trait MonkeyExecutor {
    fn execute_round(self, worry_divisor: i64, worry_remainder: i64) -> Self;
}
impl MonkeyExecutor for Vec<Monkey> {
    fn execute_round(mut self, worry_divisor: i64, worry_remainder: i64) -> Vec<Monkey> {
        for i in 0..self.len() {
            let items = self[i].items.clone();
            self[i].items.clear();
            self[i].throw_count  += items.len() as i64;
            for item in items {
                let worry_level = self[i].new_expression.execute(item) / worry_divisor % worry_remainder;
                let target = match worry_level % self[i].test_divisor {
                    0 => self[i].true_target,
                    _ => self[i].false_target
                } as usize;
                self[target].items.push(worry_level)
            }
        }
        self
    }
}