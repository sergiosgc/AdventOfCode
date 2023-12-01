use std::io::BufRead;
use aoc::{regex::ExtractTuple3, valve::Valve};
use std::collections::HashMap;

use regex::Regex;
pub fn solve(time: i64, flow: i64, released: i64, valve: String, map: &mut HashMap::<String, Valve>) -> Option<i64> {
    if time == 0 { return Some(released); }
    let current_valve = map.get(&valve).unwrap().clone();
    if current_valve.on && current_valve.best_on.is_some() && current_valve.best_on.as_ref().unwrap().time < time && current_valve.best_on.as_ref().unwrap().flow > flow && current_valve.best_on.as_ref().unwrap().released > released { return None; }
    if !current_valve.on && current_valve.best_off.is_some() && current_valve.best_off.as_ref().unwrap().time < time && current_valve.best_off.as_ref().unwrap().flow > flow && current_valve.best_off.as_ref().unwrap().released > released { return None; }
    /*if current_valve.on {
        map.get_mut(&valve).unwrap().best_on = Some(aoc::visit::Visit{ time, flow, released });
    } else {
        map.get_mut(&valve).unwrap().best_off = Some(aoc::visit::Visit{ time, flow, released });
    }*/

    let mut result: Option<i64> = None;
    if !current_valve.on && current_valve.flow_rate > 0 {
        map.get_mut(&valve).unwrap().on = true;
        result = match solve(time-1, flow + current_valve.flow_rate, released + flow, valve.clone(), map) {
            Some(candidate) => match result {
                Some(result) => Some(std::cmp::max(candidate, result)),
                None => Some(candidate)
            }
            None => result
        };
        map.get_mut(&valve).unwrap().on = false;
    } else {
        result = match solve(time-1, flow, released + flow, valve.clone(), map) {
            Some(candidate) => match result {
                Some(result) => Some(std::cmp::max(candidate, result)),
                None => Some(candidate)
            }
            None => result
        };
    }
    for exit in current_valve.exits.into_iter() {
        result = match solve(time-1, flow, released + flow, exit, map) {
            Some(candidate) => match result {
                Some(result) => Some(std::cmp::max(candidate, result)),
                None => Some(candidate)
            }
            None => result
        }
    }
    result
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input: HashMap::<String, Valve> = std::io::BufReader::new(std::io::stdin())
        .lines().filter_map(std::io::Result::ok)
        .map(|line| 
            <regex::Captures<'_> as ExtractTuple3<String, String, String>>::extract_tuple(
                match
                Regex::new(r#"^Valve (..) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)"#).unwrap()
                .captures(&line)
                {
                    None => panic!("{} does not match", line),
                    Some(c) => c

                }
            )
        )
        .map(|(valve, flow, exits): (String, String, String)| (valve,flow.parse::<i64>().unwrap(), exits.replace(" ", "").split(",").map(|s| s.to_string()).collect::<Vec<String>>()))
        .fold(HashMap::<String, Valve>::new(), |mut map, (valve, flow, exits)| {
            map.insert(valve, Valve{ flow_rate: flow, exits: exits, ..Default::default()});
            map
        });
    println!("{:#?}", solve(30, 0, 0, "AA".to_string(), &mut input));
    Ok(())
}