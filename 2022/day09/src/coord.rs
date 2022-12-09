#[derive(Copy, Clone, PartialEq, Eq, Hash )]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn new(x: i64, y: i64) -> Coord { Coord{ x: x, y: y} }
    pub fn length(self) -> f64 {
        (self.x.pow(2) as f64 + self.y.pow(2) as f64).sqrt()
    }
    pub fn unit(self) -> Coord {
        if self.x != 0 && self.y != 0 { panic!("Unimplemented")};
        Coord::new(
            if self.x == 0 { 0 } else { self.x / self.x.abs() },
            if self.y == 0 { 0 } else { self.y / self.y.abs() }
        )
    }
}
impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Coord({},{})", self.x, self.y) }
}
impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}
impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}
impl std::ops::Mul<i64> for Coord {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Coord { x: self.x * rhs, y: self.y * rhs }
    }
}