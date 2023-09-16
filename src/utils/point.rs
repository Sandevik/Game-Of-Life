use super::{Coordinates, Matrix};


#[derive(Debug, Clone)]
pub struct Point {
    pub coordinates: Coordinates,
    pub alive: bool, 
    pub neighbours_count: u8,
}

impl Point {
    pub fn new(coordinates: Coordinates, alive: bool) -> Point {
        Point {
            coordinates, 
            alive,
            neighbours_count: 0,
        }
    }
    
    pub fn calculate_alive_neighbours(&mut self, matrix: &Matrix, size: u8) -> () {
        let mut count: u8 = 0;
        let top: bool = self.coordinates.y > 0 && matrix[(self.coordinates.y - 1) as usize][self.coordinates.x as usize].alive == true;
        let top_left: bool = self.coordinates.y > 0 && self.coordinates.x > 0 && matrix[(self.coordinates.y - 1) as usize][(self.coordinates.x - 1) as usize].alive == true;
        let top_right: bool = self.coordinates.y > 0 && self.coordinates.x < size && matrix[(self.coordinates.y - 1) as usize][(self.coordinates.x + 1) as usize].alive == true;
        
        let bottom: bool = self.coordinates.y < size && matrix[(self.coordinates.y + 1) as usize][self.coordinates.x as usize].alive == true;
        let bottom_left: bool = self.coordinates.y < size && self.coordinates.x > 0 && matrix[(self.coordinates.y + 1) as usize][(self.coordinates.x - 1) as usize].alive == true;
        let bottom_right: bool = self.coordinates.y < size && self.coordinates.x < size && matrix[(self.coordinates.y + 1) as usize][(self.coordinates.x + 1) as usize].alive == true;
        
        let left: bool = self.coordinates.x > 0 && matrix[self.coordinates.y as usize][(self.coordinates.x - 1) as usize].alive == true;
        let right: bool = self.coordinates.x < size && matrix[self.coordinates.y as usize][(self.coordinates.x + 1) as usize].alive == true;
        if top {
            count += 1;
        }
        if bottom {
            count += 1;
        }
        if left {
            count += 1;
        }
        if right {
            count += 1;
        }
        if top_left {
            count += 1;
        }
        if top_right {
            count += 1;
        }
        if bottom_left {
            count += 1
        }
        if bottom_right {
            count += 1;
        }
        self.neighbours_count = count;
    }


}