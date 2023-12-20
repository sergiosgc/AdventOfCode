use std::{io::BufRead, collections::HashMap};
use consume_iterator::ConsumeIterator;
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
    pub fn apply(&self, part: &Part) -> Vec<(Part, Option<String>)> {
        if self.lhs.is_none() {
            vec![(part.clone(), Some(self.workflow.clone()))]
        } else {
            let value = match self.lhs.clone().unwrap().as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                _ => part.s,
            };
            match self.operator {
                Some(operator) => 
                    match operator {
                        Operator::LessThan => vec![(value.0, self.rhs.unwrap() - 1), (self.rhs.unwrap(), value.1)],
                        Operator::GreaterThan => vec![(self.rhs.unwrap()+1, value.1), (value.0, self.rhs.unwrap())]
                    }
                None => vec![(value.0, value.1)]
            }.into_iter()
            .filter(|range| range.1 >= range.0)
            .map(|range| {
                let mut part_slice = part.clone();
                match self.lhs.clone().unwrap().as_str() {
                    "x" => { part_slice.x = range; }
                    "m" => { part_slice.m = range; }
                    "a" => { part_slice.a = range; }
                    _ => { part_slice.s = range; }
                };
                (
                    part_slice, 
                    if self.operator.is_none() || match self.operator.unwrap() {
                        Operator::LessThan => range.1 < self.rhs.unwrap(),
                        Operator::GreaterThan => range.0 > self.rhs.unwrap()
                    } { Some(self.workflow.clone()) } else { None }
                )
            })
            .collect::<Vec<(Part, Option<String>)>>()
        }
    }
    pub fn apply_conditions(conditions: &Vec<Condition>, part: &Part) -> Vec<(String, Part)> {
        let mut result: Vec<(String, Part)> = Vec::new();
        let mut current_part = Some(part.clone());
        let mut conditions_iter = conditions.into_iter();
        while current_part.is_some() {
            match &conditions_iter.next().unwrap().apply(&current_part.unwrap())[..] {
                [(part_a, workflow_a), (part_b, workflow_b)] => {
                    result.push((workflow_a.clone().unwrap(), part_a.clone()));
                    match workflow_b {
                        Some(_) => panic!("Workflow should be none"),
                        None => { current_part = Some(part_b.clone()); }
                    };
                },
                [(part, workflow)] => {
                    result.push((workflow.clone().unwrap(), part.clone()));
                    current_part = None;

                },
                _ => panic!("Unexpected size of Condition::apply() result")
            }
        }

        result
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Part {
    pub x: (i64, i64),
    pub m: (i64, i64),
    pub a: (i64, i64),
    pub s: (i64, i64),
}
pub fn solve(workflows: &HashMap<String, Vec<Condition>>) -> Vec<Part> {
    let mut result: Vec<Part> = Vec::new();
    let mut work_queue: Vec<(String, Part)> = vec![("in".to_string(), Part { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000) })];
    while work_queue.len() > 0 {
        let (workflow, part) = work_queue.pop().unwrap();
        Condition::apply_conditions(&workflows.get(&workflow).unwrap(), &part)
        .into_iter()
        .map(|(workflow, part)| match workflow.as_str() {
            "A" => { result.push(part); },
            "R" => {},
            _ => { work_queue.push((workflow, part)); },
        })
        .consume();
    }
    result
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workflows = {
        let workflow_re = Regex::new(r#"^(?P<label>[a-z]+)\{(?P<workflow>.*)\}$"#).unwrap();

        let mut result = HashMap::<String, Vec<Condition>>::new();
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
                acc.insert(label, conditions);
            }
            acc
        });
        result
    };
    println!("{:#?}", solve(&workflows)
    .iter()
    .map(|part| (part.x.1 - part.x.0 + 1) * (part.m.1 - part.m.0 + 1) * (part.a.1 - part.a.0 + 1) * (part.s.1 - part.s.0 + 1))
    .sum::<i64>()
    );
    Ok(())
}

