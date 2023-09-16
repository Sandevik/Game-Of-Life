mod utils;
use utils::{board::*, shapes::Shapes};

use crate::utils::Coordinates;

#[tokio::main]
async fn main() {
    let mut board: Board = Board::new(16);
    
    //Blinker
    board.add_shape(Shapes::blinker(Coordinates { x: 0, y: 0 }));
    
    //Beacon
    board.add_shape(Shapes::beacon(Coordinates {x:6, y:4}));
    
    //space ship
    board.add_shape(Shapes::space_ship(Coordinates { x: 5, y: 10 }));


    board.start().await;
}



