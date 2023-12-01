#[derive(Clone,Debug,Default)]
pub struct Valve {
    pub on: bool,
    pub flow_rate: i64,
    pub exits: Vec<String>,
    pub best_on: Option<crate::visit::Visit>,
    pub best_off: Option<crate::visit::Visit>
}