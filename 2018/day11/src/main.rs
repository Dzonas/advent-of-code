mod lib;

use crate::lib::Grid;

const GRID_SIZE: usize = 300;
const SQUARE_SIZE: usize = 3;

fn main() {
    let serial_number = 8868;
    let grid = Grid::new(serial_number, GRID_SIZE);
    let square_with_larges_power = grid.square_with_largest_power(SQUARE_SIZE);

    println!("Square 3x3 with largest power:");
    println!("{:?}", square_with_larges_power.top_left);
    println!("Power level {}", square_with_larges_power.power_level);

    let best_square = grid.largest_power();

    println!("Best square:");
    println!("{:?}", best_square.top_left);
    println!("Size {}", best_square.size);
    println!("Power level {}", best_square.power_level);
}
