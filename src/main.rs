mod utils;
use utils::board::*;

use crate::utils::Coordinates;

#[tokio::main]
async fn main() {
    let mut board: Board = Board::new(16);
    
    //Blinker
    board.toggle_point_life(Coordinates::new(1, 0));
    board.toggle_point_life(Coordinates::new(1, 1));
    board.toggle_point_life(Coordinates::new(1, 2));
    
    //Beacon
    board.toggle_point_life(Coordinates::new(4, 4)); 
    board.toggle_point_life(Coordinates::new(5, 4));  
    board.toggle_point_life(Coordinates::new(4, 5));
    board.toggle_point_life(Coordinates::new(5, 5));
    board.toggle_point_life(Coordinates::new(6, 6));  
    board.toggle_point_life(Coordinates::new(7, 6));  
    board.toggle_point_life(Coordinates::new(6, 7));
    board.toggle_point_life(Coordinates::new(7, 7));
    
    //space ship
    board.toggle_point_life(Coordinates::new(7, 10));
    board.toggle_point_life(Coordinates::new(8, 11));
    board.toggle_point_life(Coordinates::new(8, 12));
    board.toggle_point_life(Coordinates::new(7, 12));
    board.toggle_point_life(Coordinates::new(6, 12));

    board.start().await;
}



