use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct Valve {
    pub flow_rate: i64,
    pub tunnels: Vec<(i64, String)>,
    pub closed: bool,
    pub best_visits: HashMap::<i64, i64>,
}