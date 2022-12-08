use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Inode {
    pub files: HashMap<String, i64>,
    pub dirs: HashMap<String, Inode>,
}
impl Inode {
    pub fn new() -> Inode { Inode { files: HashMap::new(), dirs: HashMap::new() }}
    pub fn size(self) -> i64 {
        self.files.into_iter().map(|(_name, size)| size).sum::<i64>()
            +self.dirs.into_iter().map(|(_name, dir)| dir.size()).sum::<i64>()
    }
    pub fn sizes(self) -> Vec<i64> {
        let mut result = vec![self.clone().size()];
        for (_dirname, dir) in self.dirs {
            result.append(&mut dir.sizes())
        }
        result
    }
}
