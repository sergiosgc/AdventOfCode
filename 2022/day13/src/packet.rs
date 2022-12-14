use std::cmp::Ordering;

use regex::Regex;

#[derive(Debug,Clone,PartialEq,Eq,Ord)]
pub enum Packet {
    Value(i64),
    List(Vec<Packet>)
}
impl Packet {
    pub fn from_string(s: String) -> Packet {
        if s.len() == 0 { panic!("Empty string"); }
        let mut stack: Vec::<Packet> = Vec::new();
        for token in (PacketTokenizer { input: s }) {
            match token {
                PacketToken::ListStart => {
                    stack.push(Packet::List(Vec::new()));
                },
                PacketToken::ListEnd => {
                    let current = stack.pop().unwrap();
                    match stack.len() {
                        0 => { return current; }
                        _ => {
                            let mut head = stack.pop().unwrap();
                            match head {
                                Packet::Value(_) => panic!("Unmatched ]"),
                                Packet::List(ref mut list) => { list.push(current.clone()); }
                            }
                            stack.push(head)
                        }
                    }

                },
                PacketToken::Value(v) => {
                    let mut head = stack.pop().unwrap();
                    match head {
                        Packet::Value(_) => panic!("Unmatched ]"),
                        Packet::List(ref mut list) => { list.push(Packet::Value(v)); }
                    }
                    stack.push(head)
                },
            }
        }
        panic!("Unmatched [");
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Value(left), Packet::Value(right)) => Some(left.cmp(right)),
            (Packet::List(_left), Packet::Value(right)) => self.partial_cmp(&Packet::List(vec![Packet::Value(*right)])),
            (Packet::Value(left), Packet::List(right)) => Packet::List(vec![Packet::Value(*left)]).partial_cmp(&Packet::List(right.to_vec())),
            (Packet::List(left), Packet::List(right)) => {
                let mut result: Option<Ordering> = None;
                left.iter().enumerate().map(|(i, left_item)| {
                    if result.is_none() {
                        if i >= right.len() {
                            result = Some(Ordering::Greater);
                        } else {
                            if left_item < &right[i] {
                                result = Some(Ordering::Less);
                            } else if left_item > &right[i] {
                                result = Some(Ordering::Greater)
                            }
                        }
                    }
                }).count();
                if result.is_none() {
                    result = Some(left.len().cmp(&right.len()));
                } 
                result
            }
            

        }
    }
}
#[derive(Debug)]
pub enum PacketToken {
    ListStart,
    ListEnd,
    Value(i64)
}
#[derive(Debug)]
pub struct PacketTokenizer {
    pub input: String
}
impl Iterator for PacketTokenizer {
    type Item = PacketToken;
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() == 0 { return None; }
        let input = self.input.clone();
        let captures = Regex::new(r#"^(?:(?P<list_start>\[)|(?P<list_end>\])|(?P<value>\d+))(?:,?)(?P<remainder>.*)$"#).unwrap().captures(&input).unwrap();
        match (captures.name("list_start"), captures.name("list_end"), captures.name("value"), captures.name("remainder")) {
            (Some(_capture), None, None, Some(remainder)) => {
                self.input = remainder.as_str().to_string();
                Some(PacketToken::ListStart)
            }
            (None, Some(_capture), None, Some(remainder)) => {
                self.input = remainder.as_str().to_string();
                Some(PacketToken::ListEnd)
            }
            (None, None, Some(capture), Some(remainder)) => {
                self.input = remainder.as_str().to_string();
                Some(PacketToken::Value(capture.as_str().parse::<i64>().unwrap()))
            }
            _ => panic!("Unexpected token at {}", self.input)
        }

    }

}