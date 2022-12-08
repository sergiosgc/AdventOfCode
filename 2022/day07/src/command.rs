use regex::Regex;
use itertools::Itertools;
use crate::inode::Inode;

#[derive(Debug,PartialEq,Clone)]
pub enum Command {
    LS,
}
impl Command {
    pub fn consume_input(self, line: String, cwd: &mut Inode) {
        match self {
            Command::LS => match [
                    Regex::new(r"^dir (?P<dir_name>.*)$").unwrap(),
                    Regex::new(r"^(?P<size>[0-9]+) (?P<file_name>.*)$").unwrap(),
                ].into_iter()
                .map(|regex| regex.captures(&line))
                .next_tuple::<(Option<regex::Captures>, Option<regex::Captures>)>().unwrap() {
                    (Some(capture), None) => { cwd.dirs.insert(capture.name("dir_name").unwrap().as_str().to_string(), Inode::new()); },
                    (None, Some(capture)) => { cwd.files.insert(capture.name("file_name").unwrap().as_str().to_string(), capture.name("size").unwrap().as_str().parse::<i64>().unwrap()); },
                    (None, None) => panic!("Unparseable line {}", line),
                    _ => panic!("Multiple matches for line {}", line)
                },
        };
    }
}
