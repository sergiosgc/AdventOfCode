use parse_display::{Display, FromStr};

#[derive(Clone, Display, FromStr, Debug, PartialEq, Eq)]
pub enum MoveOrder {
    #[display("{0}")]
    Forward(i64),
    #[display("L")]
    TurnLeft,
    #[display("R")]
    TurnRight
}
