use std::{io::BufRead, collections::HashMap};
use regex::Regex;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Operator {
    #[default]
    LessThan,
    GreaterThan
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Condition {
    pub lhs: Option<String>,
    pub rhs: Option<i64>,
    pub operator: Option<Operator>,
    pub workflow: String
}
impl Condition {
    pub fn apply(&self, part: &Part) -> Option<String> {
        if self.lhs.is_none() {
            Some(self.workflow.clone())
        } else {
            let value = match self.lhs.clone().unwrap().as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                _ => part.s,
            };
            if match self.operator.unwrap() {
                Operator::LessThan => value < self.rhs.unwrap(),
                Operator::GreaterThan => value > self.rhs.unwrap(),
            } {
                Some(self.workflow.clone())
            } else {
                None
            }
        }
    }
    pub fn apply_all(conditions: &Vec<Condition>, part: &Part) -> String {
        conditions.into_iter().flat_map(|c| c.apply(part)).next().unwrap()
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Part {
    pub x: i64,
    pub m: i64,
    pub a: i64,
    pub s: i64,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (workflows, parts) = {
        let workflow_re = Regex::new(r#"^(?P<label>[a-z]+)\{(?P<workflow>.*)\}$"#).unwrap();
        let part_re = Regex::new(r#"^\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}"#).unwrap();

        let mut result = (HashMap::<String, Vec<Condition>>::new(), Vec::<Part>::new());
        std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok)
        .fold(&mut result, |acc, line| {
            if workflow_re.is_match(&line) {
                let captures = workflow_re.captures(&line).unwrap();
                let label = captures.name("label").unwrap().as_str().to_string();
                let conditions = captures.name("workflow").unwrap().as_str().to_string().split(",")
                .map(|text_condition| {
                    if text_condition.contains(&['<', '>']) {
                        let mut rating = text_condition.to_string();
                        let mut operator = rating.split_off(1);
                        let value_workflow = operator.split_off(1); 
                        let (value, workflow) = value_workflow.split_once(":").unwrap();
                        Condition{
                            lhs: Some(rating),
                            rhs: Some(value.parse::<i64>().unwrap()),
                            operator: Some(if operator == "<" { Operator::LessThan } else { Operator::GreaterThan }),
                            workflow: workflow.to_string(),
                        }
                    } else {
                        Condition {
                            lhs: None,
                            rhs: None,
                            operator: None,
                            workflow: text_condition.to_string()
                        }
                    }
                })
                .collect::<Vec<Condition>>();
                acc.0.insert(label, conditions);
            } else if part_re.is_match(&line) {
                let captures = part_re.captures(&line).unwrap();
                acc.1.push(match (captures.name("x"),captures.name("m"),captures.name("a"),captures.name("s")) {
                    (Some(x), Some(m), Some(a), Some(s)) => Part { 
                        x: x.as_str().parse::<i64>().unwrap(),
                        m: m.as_str().parse::<i64>().unwrap(),
                        a: a.as_str().parse::<i64>().unwrap(),
                        s: s.as_str().parse::<i64>().unwrap()
                    },
                    _ => panic!("Failed capture")
                });
            }
            acc
        });
        result
    };
    println!("{:#?}", parts.iter().flat_map(|part| {
        let mut workflow = "in".to_string();
        while workflow != "A" && workflow != "R" {
            workflow = Condition::apply_all(&workflows.get(&workflow).unwrap(), &part);
        }
        if workflow == "A" {
            Some(part)
        } else {
            None
        }
    })
    .map(|p| p.x + p.m + p.a + p.s)
    .sum::<i64>()
    );
    Ok(())
}
