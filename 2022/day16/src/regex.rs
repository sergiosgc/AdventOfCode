use regex::Captures;
pub trait ExtractTuple4<A,B,C,D> {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
pub trait ExtractTuple3<A,B,C> {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
pub trait ExtractTuple2<A,B> {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
pub trait ExtractTuple1<A> {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
impl<A,B,C,D> ExtractTuple4<A,B,C,D> for Captures::<'_> 
where 
    A: std::str::FromStr, <A as std::str::FromStr>::Err: std::fmt::Debug,
    B: std::str::FromStr, <B as std::str::FromStr>::Err: std::fmt::Debug,
    C: std::str::FromStr, <C as std::str::FromStr>::Err: std::fmt::Debug,
    D: std::str::FromStr, <D as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = (A, B, C, D);
    fn extract_tuple(self) -> Self::Item {
        (self.get(1).unwrap().as_str().parse::<A>().unwrap(),
         self.get(2).unwrap().as_str().parse::<B>().unwrap(),
         self.get(3).unwrap().as_str().parse::<C>().unwrap(),
         self.get(4).unwrap().as_str().parse::<D>().unwrap())
    }
}
impl<A,B,C> ExtractTuple3<A,B,C> for Captures::<'_> 
where 
    A: std::str::FromStr, <A as std::str::FromStr>::Err: std::fmt::Debug,
    B: std::str::FromStr, <B as std::str::FromStr>::Err: std::fmt::Debug,
    C: std::str::FromStr, <C as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = (A, B, C);
    fn extract_tuple(self) -> Self::Item {
        (self.get(1).unwrap().as_str().parse::<A>().unwrap(),
         self.get(2).unwrap().as_str().parse::<B>().unwrap(),
         self.get(3).unwrap().as_str().parse::<C>().unwrap())
    }
}
impl<A,B> ExtractTuple2<A,B> for Captures::<'_> 
where 
    A: std::str::FromStr, <A as std::str::FromStr>::Err: std::fmt::Debug,
    B: std::str::FromStr, <B as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = (A, B);
    fn extract_tuple(self) -> Self::Item {
        (self.get(1).unwrap().as_str().parse::<A>().unwrap(),
         self.get(2).unwrap().as_str().parse::<B>().unwrap())
    }
}
impl<A> ExtractTuple1<A> for Captures::<'_> 
where 
    A: std::str::FromStr, <A as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = A;
    fn extract_tuple(self) -> Self::Item {
        self.get(1).unwrap().as_str().parse::<A>().unwrap()
    }
}