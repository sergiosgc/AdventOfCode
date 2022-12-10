#[derive(Debug,Clone,Copy)]
pub enum Opcode {
    Noop,
    Addx(i64)
}
impl Opcode {
    pub fn noop() -> Opcode { Opcode::Noop }
    pub fn addx(x: i64) -> Opcode { Opcode::Addx(x) }
    pub fn from_str(s: &str) -> Opcode {
        let split = s.split(" ").collect::<Vec<&str>>();
        match split[0] {
            "noop" => Opcode::noop(),
            "addx" => Opcode::addx(split[1].parse::<i64>().unwrap()),
            _ => panic!("Unpexpected opcode")
        }
    }
}