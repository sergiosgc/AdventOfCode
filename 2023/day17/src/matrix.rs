use std::ops::Index;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Matrix<T> 
where T: Clone + Default
{
    pub rows: Vec<Vec<T>>,
}
impl<T> Matrix<T> 
where T: Clone + Default
{
    pub fn new(dimension: (usize, usize)) -> Matrix<T> {
        Matrix { rows: vec![vec![T::default(); dimension.1]; dimension.0] }
    }
    pub fn size(&self) -> (usize, usize) {
        ( match self.rows.get(0) { Some(row) => row.len() - 1, None => 0 }, self.rows.len() -1 )
    }
}
impl<T> Index<(usize, usize)> for Matrix<T>
where T: Clone + Default
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.rows[index.1][index.0]
    }
}
impl<T> FromIterator<((usize, usize), T)> for Matrix<T>
where T: Clone + Default
{
    fn from_iter<I: IntoIterator<Item = ((usize, usize), T)>>(iter: I) -> Self {
        let mut rows: Vec<Vec<T>> = Vec::new();
        for ((x, y), value) in iter.into_iter() {
            while rows.len() < y+1 { rows.push(Vec::new()); }
            while rows[y].len() < x+1 { rows[y].push( T::default() ); }
            rows[y][x] = value;
        }
        let width = rows.iter().map(|row| row.len() ).max().unwrap_or(0);
        for row in rows.iter_mut() { while row.len() < width { row.push(T::default())} }
        Self { rows }
    }
}