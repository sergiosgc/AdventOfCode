use regex::Captures;
pub trait ExtractTuple7<A,B,C,D,E,F,G> {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
pub trait ExtractTuple6<A,B,C,D,E,F> {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
pub trait ExtractTuple5<A,B,C,D,E> {
    type Item;
    fn extract_tuple(self) -> Self::Item;
}
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
impl<A,B,C,D,E,F,G> ExtractTuple7<A,B,C,D,E,F,G> for Captures::<'_> 
where 
    A: std::str::FromStr, <A as std::str::FromStr>::Err: std::fmt::Debug,
    B: std::str::FromStr, <B as std::str::FromStr>::Err: std::fmt::Debug,
    C: std::str::FromStr, <C as std::str::FromStr>::Err: std::fmt::Debug,
    D: std::str::FromStr, <D as std::str::FromStr>::Err: std::fmt::Debug,
    E: std::str::FromStr, <E as std::str::FromStr>::Err: std::fmt::Debug,
    F: std::str::FromStr, <F as std::str::FromStr>::Err: std::fmt::Debug,
    G: std::str::FromStr, <G as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = (A, B, C, D, E, F,G);
    fn extract_tuple(self) -> Self::Item {
        (self.get(1).unwrap().as_str().parse::<A>().unwrap(),
         self.get(2).unwrap().as_str().parse::<B>().unwrap(),
         self.get(3).unwrap().as_str().parse::<C>().unwrap(),
         self.get(4).unwrap().as_str().parse::<D>().unwrap(),
         self.get(5).unwrap().as_str().parse::<E>().unwrap(),
         self.get(6).unwrap().as_str().parse::<F>().unwrap(),
         self.get(7).unwrap().as_str().parse::<G>().unwrap())
    }
}
impl<A,B,C,D,E,F> ExtractTuple6<A,B,C,D,E,F> for Captures::<'_> 
where 
    A: std::str::FromStr, <A as std::str::FromStr>::Err: std::fmt::Debug,
    B: std::str::FromStr, <B as std::str::FromStr>::Err: std::fmt::Debug,
    C: std::str::FromStr, <C as std::str::FromStr>::Err: std::fmt::Debug,
    D: std::str::FromStr, <D as std::str::FromStr>::Err: std::fmt::Debug,
    E: std::str::FromStr, <E as std::str::FromStr>::Err: std::fmt::Debug,
    F: std::str::FromStr, <F as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = (A, B, C, D, E, F);
    fn extract_tuple(self) -> Self::Item {
        (self.get(1).unwrap().as_str().parse::<A>().unwrap(),
         self.get(2).unwrap().as_str().parse::<B>().unwrap(),
         self.get(3).unwrap().as_str().parse::<C>().unwrap(),
         self.get(4).unwrap().as_str().parse::<D>().unwrap(),
         self.get(5).unwrap().as_str().parse::<E>().unwrap(),
         self.get(6).unwrap().as_str().parse::<F>().unwrap())
    }
}
impl<A,B,C,D,E> ExtractTuple5<A,B,C,D,E> for Captures::<'_> 
where 
    A: std::str::FromStr, <A as std::str::FromStr>::Err: std::fmt::Debug,
    B: std::str::FromStr, <B as std::str::FromStr>::Err: std::fmt::Debug,
    C: std::str::FromStr, <C as std::str::FromStr>::Err: std::fmt::Debug,
    D: std::str::FromStr, <D as std::str::FromStr>::Err: std::fmt::Debug,
    E: std::str::FromStr, <E as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Item = (A, B, C, D, E);
    fn extract_tuple(self) -> Self::Item {
        (self.get(1).unwrap().as_str().parse::<A>().unwrap(),
         self.get(2).unwrap().as_str().parse::<B>().unwrap(),
         self.get(3).unwrap().as_str().parse::<C>().unwrap(),
         self.get(4).unwrap().as_str().parse::<D>().unwrap(),
         self.get(5).unwrap().as_str().parse::<E>().unwrap())
    }
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