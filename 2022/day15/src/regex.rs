use regex::Captures;

pub trait ExtractTuple {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
impl ExtractTuple for Captures::<'_> {
    type Item = (i64, i64, i64, i64);
    fn extract_tuple(self) -> Self::Item {
        (self.get(1).unwrap().as_str().parse::<i64>().unwrap(),
         self.get(2).unwrap().as_str().parse::<i64>().unwrap(),
         self.get(3).unwrap().as_str().parse::<i64>().unwrap(),
         self.get(4).unwrap().as_str().parse::<i64>().unwrap())
    }
}