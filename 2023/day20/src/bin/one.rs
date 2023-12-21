use std::{io::BufRead, collections::{HashMap, VecDeque}};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    Receiver
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Module {
    pub module_type: ModuleType,
    pub label: String,
    pub inputs: Vec<(String, bool)>,
    pub outputs: Vec<String>,
    pub state: bool
}
impl Module {
    pub fn new(module_type: &ModuleType, label: &String, outputs: &Vec<String>) -> Module {
        Module {
            module_type: module_type.clone(),
            label: label.clone(),
            inputs: Vec::new(),
            outputs: outputs.clone(),
            state: false
        }
    }
    pub fn pulse(&mut self, from: &String, high: bool) -> Vec<(String, String, bool)> {
        match self.module_type {
            ModuleType::FlipFlop => if high {
                vec![]
            } else {
                self.state = !self.state;
                self.outputs.iter().map(|to| (self.label.clone(), to.clone(), self.state)).collect()
            }
            ModuleType::Conjunction => {
                self.inputs.iter_mut().find(|input| input.0 == *from).unwrap().1 = high;
                let pulse = !self.inputs.iter().fold(true, |acc, (_, input_value)| acc && *input_value);
                self.outputs.iter().map(|to| (self.label.clone(), to.clone(), pulse)).collect()
            }
            ModuleType::Broadcast => self.outputs.iter().map(|to| (self.label.clone(), to.clone(), high)).collect(),
            ModuleType::Receiver => vec![]
        }
    }
}
pub fn pulse(machine: &mut HashMap<String,Module>) -> Vec<i64> {
    let mut queue = VecDeque::from([("button".to_string(), "broadcaster".to_string(), false)]);
    let mut result = vec![1,0];
    while let Some((from, to, high)) = queue.pop_front() {
        machine.get_mut(&to).unwrap().pulse(&from, high).iter().for_each(|pulse| { result[if pulse.2 { 1 } else { 0 }] += 1; queue.push_back(pulse.clone()); });
    }
    result
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::BufReader::new(std::io::stdin())
    .lines()
    .filter_map(std::io::Result::ok)
    .map(|line| {
        let (module_definition,outputs_string) = line.split_once(" -> ").unwrap();
        let outputs_vec = outputs_string.replace(" ", "").split(',').map(|output| output.to_string()).collect::<Vec<String>>();
        if module_definition == "broadcaster" {
            Module::new(&ModuleType::Broadcast, &module_definition.to_string(), &outputs_vec)
        } else {
            let mut module_type = module_definition.to_string();
            let module_label = module_type.split_off(1);
            Module::new( &if module_type == "%" { ModuleType::FlipFlop } else { ModuleType::Conjunction }, &module_label, &outputs_vec)
        }
    })
    .fold(HashMap::<String, Module>::new(), |mut acc, module| {
        acc.insert(module.label.clone(), module);
        acc
    });
    input.insert("rx".to_string(), Module::new(&ModuleType::Receiver, &"rx".to_string(), &vec![]));
    for (module, to_insert) in input.clone().into_iter().flat_map(|(_, module)| {
        module.outputs.into_iter().map(|output| (output, (module.label.clone(), false))).collect::<Vec<(String, (String, bool))>>()
    }) {
        input.get_mut(&module).unwrap().inputs.push(to_insert);
    }
    println!("{:#?}", (0..1000).map(|_| pulse(&mut input)).reduce(|mut a,b| { a[0] += b[0]; a[1] += b[1]; a }).unwrap().into_iter().reduce(|a,b| a*b).unwrap()) ;
    Ok(())
}
