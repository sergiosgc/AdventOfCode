use crate::opcode::Opcode;

#[derive(Debug,Clone,Copy)]
pub struct CPU {
    pub x: i64,
}
impl CPU {
    pub fn new() -> CPU { CPU{ x: 1 } }
    pub fn execute(mut self, instruction: Opcode) -> CPU {
        match instruction {
            Opcode::Noop => {},
            Opcode::Addx(x) => { self.x += x; }
        }
        self
    }
}
#[derive(Debug,Clone)]
pub struct CycleIterator {
    cpu: CPU,
    wait_cycles: Vec<CPU>,
    program: Vec<Opcode>,
    ended: bool,
}
impl CycleIterator {
    pub fn new(program: Vec<Opcode>) -> CycleIterator {
        let mut p = program.clone();
        p.reverse();
        CycleIterator { cpu: CPU::new(), wait_cycles: Vec::new(), program: p, ended: false }
    }
    pub fn opcode_length(opcode: Opcode) -> i64 {
        match opcode {
            Opcode::Noop => 1,
            Opcode::Addx(_x) => 2,
        }
    }
}
impl Iterator for CycleIterator {
    type Item = CPU;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ended {
            true => None,
            false => match self.wait_cycles.pop() {
                Some(cpu) => Some(cpu),
                None => match self.program.pop() {
                    None => {
                        self.ended = true;
                        Some(self.cpu)
                    },
                    Some(opcode) => {
                        for _i in 0..CycleIterator::opcode_length(opcode) {
                            self.wait_cycles.push(self.cpu.clone())
                        }
                        self.cpu = self.cpu.execute(opcode);
                        Some(self.wait_cycles.pop().unwrap())
                    }


                }
            }
        }
    }
}