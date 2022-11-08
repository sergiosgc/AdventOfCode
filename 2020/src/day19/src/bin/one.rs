use std::{io::BufRead, collections::HashMap};

use regex::Regex;
#[derive(Clone,Debug)]
struct Rule {
    either: Option<(Box<Rule>,Box<Rule>)>,
    sequence: Option<Vec<i32>>,
    char: Option<char>
}
#[derive(Clone,Debug)]
struct Rules(HashMap<i32, Rule>);
impl Rule {
    fn from_string(s: String) -> Rule {
        let captures = match Regex::new(r#"^(?:(?P<either>[0-9 ]+[|][0-9 ]+)|(?P<sequence>[0-9 ]+)|(?:"(?P<char>[a-z])"))$"#).unwrap().captures(&s) {
            Some(c) => c,
            None => panic!("Unable to parse: {}", s)
        };
        match (captures.name("either"), captures.name("sequence"), captures.name("char")) {
            (Some(either), None, None) => {
                let either_capture = Regex::new(r#"^(?P<eitherleft>[0-9 ]+)[|](?P<eitherright>[0-9 ]+)$"#).unwrap().captures(either.as_str()).unwrap();
                match (either_capture.name("eitherleft"), either_capture.name("eitherright")) {
                    (Some(left), Some(right)) => Rule {
                        either: Some((Box::new(Rule::from_string(left.as_str().to_string())), Box::new(Rule::from_string(right.as_str().to_string())))),
                        sequence: None,
                        char: None
                    },
                    _ => panic!("Unable to parse: {}", either.as_str())
                }
            },
            (None, Some(sequence), None) => {
                let sequence_re = Regex::new(r#" *(?P<index>[0-9]+)"#).unwrap();
                let sequence_capture = sequence_re.captures_iter(sequence.as_str());
                Rule{
                    either: None,
                    sequence: Some(sequence_capture
                        .map(|c| c.name("index"))
                        .filter(|index| index.is_some())
                        .map(|index| index.unwrap().as_str().parse::<i32>().unwrap())
                        .collect()),
                    char: None
                }
            },
            (None, None, Some(char)) => {
                Rule {
                    either: None,
                    sequence: None,
                    char: Some(char.as_str().chars().next().unwrap())
                }
            },
            _ => panic!("Unable to parse: {}", s),
        }
    }
    fn regex(&self, rules: &Rules) -> String {
        match (self.either.as_ref(), self.sequence.as_ref(), self.char) {
            (Some(either), None, None) => {
                format!("({}|{})", either.0.regex(rules), either.1.regex(rules))
            },
            (None, Some(sequence), None) => {
                sequence.into_iter()
                    .map(|index| rules.0[&index].regex(rules))
                    .collect::<Vec<String>>()
                    .join("")
            },
            (None, None, Some(char)) => {
                char.to_string()
            },
            _ => panic!("Unexpected")
        }
    }
}
impl Rules {
    fn from_strings(s: Vec<String>) -> Rules {
        let re = Regex::new(r#"^(?P<index>[0-9]*): (?P<rule>.*)$"#).unwrap();
        Rules {
            0: s.into_iter()
                .map( |l| match re.captures(&l.clone()) {
                    None => None,
                    Some(c) => Some((c.name("index").unwrap().as_str().parse::<i32>().unwrap(), c.name("rule").unwrap().as_str().to_string()))
                })
                .filter(|r| r.is_some())
                .map(|r| r.unwrap())
                .fold(HashMap::<i32, Rule>::new(), |mut acc, r| {
                    acc.insert(r.0, Rule::from_string(r.1));
                    acc
                })

        }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    let rules = Rules::from_strings(input.clone());
    let valid = Regex::new(&format!("^(?:{})$", rules.0[&0].regex(&rules))).unwrap();
    let input_re = Regex::new(r#"^(a|b)"#).unwrap();
    println!("{:#?}", input.into_iter()
        .map(|l| if input_re.is_match(&l) { Some(l) } else { None })
        .filter(|l| l.is_some())
        .map(|l| l.unwrap())
        .map(|l| if valid.is_match(&l) { Some(l) } else { None })
        .filter(|l| l.is_some())
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .len()
    );
    Ok(())
}
