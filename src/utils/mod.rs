use self::point::Point;
pub mod board;
pub mod point;

pub type Matrix = Vec<Row>;
pub type Row = Vec<Point>;

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub x: u8,
    pub y: u8,
}

impl Coordinates {
    pub fn new(x: u8, y: u8) -> Coordinates {
        Coordinates { x: x, y: y }
    }
}




