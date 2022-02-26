mod lib;

use crate::lib::{parse, Grid};

fn main() {
    let input = include_str!("../input");
    let data = parse(input);
    let grid = Grid::new(data);

    println!("Largest, non-infinite area: {}", grid.biggest_non_infinite_area());

    let distance = 10000;
    println!("Size of the region < {}: {}", distance, grid.area_with_equal_distance(distance));
}
