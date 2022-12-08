use regex::Regex;
use itertools::Itertools;
use crate::command::Command;
use crate::inode::Inode;


#[derive(Debug,Clone)]
pub struct Shell {
    pub root: Inode,
    pub cwd: String,
    pub current_command: Command,
}
impl Shell {
    pub fn new() -> Shell { 
        Shell { root: Inode::new(), cwd: "/".to_string(), current_command: Command::LS }
    }
    pub fn consume_input(mut self, line: String) -> Shell {
        match [
                Regex::new(r"^\$ cd (?P<next_cwd>.*)$").unwrap(),
                Regex::new(r"^\$ ls$").unwrap(),
                Regex::new(r"^[^$].*$").unwrap(),
            ].into_iter()
            .map(|regex| regex.captures(&line))
            .next_tuple::<(Option<regex::Captures>, Option<regex::Captures>, Option<regex::Captures>)>().unwrap() {
                (Some(capture), None, None) => self.cwd = match capture.name("next_cwd").unwrap().as_str() {
                    "/" => "/".to_string(),
                    ".." => Regex::new(r"^(.*)/[^/]*$").unwrap().replace(&self.cwd, "$1").to_string(),
                    cwd @ _ => self.cwd + "/" + cwd,
                },
                (None, Some(_capture), None) => self.current_command = Command::LS,
                (None, None, Some(_capture)) => {
                    let cmd = self.current_command.clone();
                    let mut cwd = &mut self.root;
                    for dir in self.cwd.split("/") {
                        match dir {
                            "" => {},
                            dir @ _ => cwd = cwd.dirs.get_mut(dir).unwrap(),
                        }
                    }
                    cmd.consume_input(line, cwd);
                },
                (None, None, None) => panic!("Unparseable line {}", line),
                _ => panic!("Multiple matches for line {}", line)
            };
        self
    }
}
