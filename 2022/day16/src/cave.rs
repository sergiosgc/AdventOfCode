use std::collections::HashMap;
use regex::Regex;

use crate::valve::Valve;
use crate::regex::ExtractTuple3;

#[derive(Clone,Debug)]
pub struct Cave {
    pub valves: HashMap<String, Valve>,
    pub current_position: String,
    pub time_left: i64,
    pub pressure_released: i64
}
impl Cave {
    pub fn from_string<I>(lines: I) -> Cave 
    where I: IntoIterator<Item=String>
    {
        let mut result = Cave {
            valves: lines.into_iter().map(|line| {
                    let (label, flow_rate, aggregated_tunnels): (String, i64, String) = Regex::new(r#"Valve ([^ ]+) has flow rate=(\d+); (?:tunnel|tunnels) (?:lead|leads) to (?:valve|valves) (.*)"#).unwrap()
                        .captures(&line).unwrap()
                        .extract_tuple();
                    (
                        label, 
                        Valve {
                            flow_rate,
                            tunnels: [
                                aggregated_tunnels.replace(' ', "").split(',').map(str::to_string).map(|s| (1,s)).collect::<Vec<(i64,String)>>(),
                                aggregated_tunnels.replace(' ', "").split(',').map(str::to_string).map(|s| format!("{}-open",s)).map(|s| (2,s)).collect::<Vec<(i64, String)>>()
                            ].into_iter().flatten().collect::<Vec<(i64, String)>>(),
                            closed: true,
                            best_visits: HashMap::new(),
                        }
                    )
                })
                .fold(HashMap::<String, Valve>::new(), |mut valves, (label, valve)| { 
                    if valve.flow_rate > 0 {
                        let mut open_valve = valve.clone();
                        open_valve.closed = false;
                        valves.insert(format!("{}-open", label), open_valve); 
                    }
                    valves.insert(label, valve); 
                    valves 
                }),
            current_position: "AA".to_string(),
            time_left: 30,
            pressure_released: 0
        };
        let result_valves = result.valves.clone();
        for (_label, mut valve) in result.valves.iter_mut() {
            valve.tunnels = valve.tunnels
                .iter()
                .filter(|&(_weight, tunnel)| result_valves.get(tunnel).is_some())
                .map(|pair| (pair.0, pair.1.clone()))
                .collect::<Vec<(i64, String)>>();
            valve.tunnels = [ valve.tunnels.iter().filter(|s| Regex::new("open").unwrap().is_match(&s.1)).map(|(a,b)| (a.clone(), b.clone())).collect::<Vec<(i64, String)>>(), 
                              valve.tunnels.iter().filter(|s| !Regex::new("open").unwrap().is_match(&s.1)).map(|(a,b)| (a.clone(), b.clone())).collect::<Vec<(i64, String)>>() ].into_iter().flatten().collect::<Vec<(i64, String)>>();
        }
        result
    }
    pub fn recursive_pressure(&mut self, valve_label: String, time_left: i64, released_so_far: i64, open_valves: Vec::<String>) -> Option<(i64, Vec<(i64, String)>)> {
        if self.valves
            .get(&valve_label).unwrap()
            .best_visits.iter()
            .any(|(visit_time_left, visit_released_so_far)| visit_time_left == &time_left && visit_released_so_far >= &released_so_far) {
                if valve_label == "EE"  && time_left == 17 {
                    for _i in 0..(30-time_left) { print!(" "); }
                    println!("Not visiting {} with {}m left. Released {}. open_valves: {:?}, because of: {:?}", valve_label.clone(), time_left, released_so_far, open_valves.clone(), 
                    self.valves
            .get(&valve_label).unwrap()
            .best_visits
                );
                }
                None
            } else if time_left == 0 { 
                    Some((released_so_far, vec![(released_so_far, valve_label.clone())].to_vec())) 
            } else {
                let valve = self.valves.get_mut(&valve_label).unwrap();
                let next_released_so_far = released_so_far + if valve.closed { 0 } else { valve.flow_rate * time_left };
                let next_open_valves = if valve.closed { open_valves.clone() } else { [ open_valves.clone(), vec![valve_label.clone()] ].iter().flatten().map(|s| s.clone()).collect::<Vec<String>>() };
                let result = valve.tunnels.clone().iter()
                    .filter(|(time, _tunnel_destination_label)| time <= &time_left)
                    .filter(|(_time, tunnel_destination_label)| !next_open_valves.contains(tunnel_destination_label))
                    .fold(vec![(time_left, valve_label.clone())].to_vec(), |mut res, dest| { res.push((dest.0, dest.1.clone())); res })
                    .iter()
                    .flat_map(|(time, tunnel_destination_label)| {
                        self.recursive_pressure(
                        tunnel_destination_label.to_string(),
                        time_left - time,
                        next_released_so_far,
                        next_open_valves.clone()
                    )
                })
                .max();
                self.valves.get_mut(&valve_label).unwrap().best_visits.insert(time_left, released_so_far);
                if valve_label == "EE" && time_left == 17{
                    for _i in 0..(30-time_left) { print!(" "); }
                    println!("Visiting {} with {}m left. Released {}. open_valves: {:?}", valve_label.clone(), time_left, released_so_far, open_valves.clone());
                    for _i in 0..(30-time_left) { print!(" "); }
                    println!("result is {:?}", result);
                }
                match result {
                    None => None,
                    Some((pressure, valves)) => Some((pressure, [ vec![(released_so_far, valve_label.clone())], valves].iter().flatten().map(|(r, s)| (r.clone(), s.clone())).collect::<Vec<(i64, String)>>()))
                }
            }


    }
}