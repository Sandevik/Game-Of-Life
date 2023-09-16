use tokio::time::{sleep_until, Instant, Duration};

use super::{Matrix, Row, point::Point, Coordinates};

#[derive(Debug, Clone)]
pub struct Board {
    pub matrix: Matrix,
    tick: u32,
    pub size: u8,
    all_dead: bool,
}

impl Board {
    pub fn new(size: u8) -> Board {
        let mut matrix: Matrix = Matrix::new();
        for y in 0..=size {
            let mut row: Row = Row::new();
            for x in 0..=size {
                row.push(Point::new(super::Coordinates::new(x, y), false))
            }
            matrix.push(row);
        }

        Board {
            matrix,
            tick: 0,
            size,
            all_dead: false,
        }
    }

    pub fn toggle_point_life(&mut self, coordinates: Coordinates) -> () {
        self.matrix[coordinates.y as usize][coordinates.x as usize].alive = !self.matrix[coordinates.y as usize][coordinates.x as usize].alive;
    }

    pub async fn start(&mut self) -> () {
        loop {
            self.show();
            sleep_until(Instant::now() + Duration::from_millis(1000)).await;
            //if self.check_all_dead() {break;}
            self.next_round();
            
        }
    }

    pub fn next_round(&mut self) -> () {
        self.tick += 1;
        let matrix: Matrix = self.matrix.clone();
        for y in 0..=self.size {
            for x in 0..=self.size {
                self.matrix[x as usize][y as usize].calculate_alive_neighbours(&matrix, self.size);
                let point: &mut Point = &mut self.matrix[x as usize][y as usize];
                match point.alive {
                    true => {
                        if point.neighbours_count < 2 || point.neighbours_count > 3 {
                            point.alive = false;
                            //println!("Point at: {}, {} died", point.coordinates.x, point.coordinates.y)
                        }
                    },
                    false => {
                        if point.neighbours_count == 3 {
                            point.alive = true;
                            //println!("Point at: {}, {} respawned", point.coordinates.x, point.coordinates.y)

                        }
                    }
                }
            }
        }
    }

    pub fn show(&self) -> () {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut string: String = String::new();
        for _ in 1..=self.size {
            string.push_str("__")
        }
        string.push_str("_\n");
        for y in 0..self.size {
            string.push('|');
            for x in 0..self.size {   
                if self.matrix[y as usize][x as usize].alive == true {
                    string.push_str("X ")
                } else {
                    if x == self.size - 1 {
                        string.push_str(" |")

                    } else {
                        string.push_str("  ")
                    }
                }
                
            }
            string.push('\n');
        }

        for _ in 1..=self.size {
            string.push_str("__")
        }
        string.push_str("_\n");
        
        println!("{string}");

    }

    fn check_all_dead(&self) -> bool {
        let mut all_dead = true;
        'outer: for y in 0..self.size {
            for x in 0..self.size {
                if self.matrix[y as usize][x as usize].alive {
                    all_dead = true;
                    break 'outer;
                }
            }
        }
        all_dead
    }
    


}