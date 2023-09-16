use super::Coordinates;


pub struct Shapes {}
pub type Shape = Vec<Coordinates>;
impl Shapes {

    pub fn blinker(coordinates: Coordinates) -> Shape {
        return vec![
            Coordinates::new(coordinates.x + 1, coordinates.y + 0),
            Coordinates::new(coordinates.x + 1, coordinates.y + 1), 
            Coordinates::new(coordinates.x + 1, coordinates.y + 2)
        ]
    }

    pub fn beacon(coordinates: Coordinates) -> Shape {
        let x = coordinates.x;
        let y = coordinates.y;
        return vec![
            Coordinates::new(0 + x, 0 + y),
            Coordinates::new(1 + x, 0 + y),
            Coordinates::new(0 + x, 1 + y),
            Coordinates::new(1 + x, 1 + y),
            Coordinates::new(2 + x, 2 + y),
            Coordinates::new(3 + x, 2 + y),
            Coordinates::new(2 + x, 3 + y),
            Coordinates::new(3 + x, 3 + y)
        ]
    }

    pub fn space_ship(coordinates: Coordinates) -> Shape {
        let x = coordinates.x;
        let y = coordinates.y;
        return vec![
            Coordinates::new(1 + x, 0 + y),
            Coordinates::new(2 + x, 1 + y),
            Coordinates::new(2 + x, 2 + y),
            Coordinates::new(1 + x, 2 + y),
            Coordinates::new(0 + x, 2 + y)
        ]
    }
    
}