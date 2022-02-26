use std::{i32, cmp::min};

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

#[derive(Debug, PartialEq)]
pub struct Square {
    pub top_left: Position,
    pub size: usize,
    pub power_level: i32,
}

impl Square {
    fn new(top_left: Position, size: usize, power_level: i32) -> Square {
        Square { top_left, size, power_level }
    }
}

pub struct Grid {
    grid_size: usize,
    grid: Vec<i32>,
}

impl Grid {
    pub fn new(serial_number: u32, grid_size: usize) -> Grid {
        let mut grid = Vec::with_capacity(grid_size * grid_size);

        // Calculate power level for each power cell
        for i in 0..grid_size {
            for j in 0..grid_size {
                let pos = Position::new(j, i);
                let power_level = Grid::get_power_level(pos, serial_number);
                grid.push(power_level);
            }
        }

        Grid { grid_size, grid }
    }

    ///
    /// Returns power level of power cell in given position in the grid.
    ///
    fn get(&self, pos: Position) -> i32 {
        self.grid[pos.y * self.grid_size + pos.x]
    }

    ///
    /// Calculates power level for power cell in given position,
    /// on the grid with given serial number.
    ///
    fn get_power_level(pos: Position, serial_number: u32) -> i32 {
        let rack_id = pos.x as i32 + 10i32;
        let mut power_level = rack_id * pos.y as i32 + serial_number as i32;
        power_level *= rack_id;
        power_level = (power_level / 100) % 10 - 5;

        power_level
    }

    ///
    /// Calculates total power level for all power cells in the square
    /// with given top left corner and it's size.
    ///
    fn get_square_power_level(&self, top_left: Position, square_size: usize) -> i32 {
        let mut total_power_level = 0;

        for i in 0..square_size {
            for j in 0..square_size {
                total_power_level += self.get(Position::new(top_left.x + j, top_left.y + i));
            }
        }

        total_power_level
    }

    ///
    /// Returns square of power cells, which has biggest total power from all
    /// possible squares, with given size, in the grid.
    ///
    pub fn square_with_largest_power(&self, square_size: usize) -> Square {
        let n_squares_in_row = (self.grid_size - square_size + 1) as usize;

        let mut power_level = i32::MIN;
        let mut position = Position::new(0, 0);

        for i in 0..n_squares_in_row {
            for j in 0..n_squares_in_row {
                let temp = self.get_square_power_level(Position::new(j, i), square_size);

                if temp > power_level {
                    power_level = temp;
                    position = Position::new(j, i);
                }
            }
        }

        Square::new(position, square_size, power_level)
    }

    ///
    /// Returns square which has biggest total power from all possible squares,
    /// i.e. all possible sizes and top left corners.
    ///
    pub fn largest_power(&self) -> Square {
        let mut top_square = Square::new(Position::new(0, 0), 0, i32::MIN);

        // Move on the grid cell by cell
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                // Each larger square, with the same top left corner, has the total power of
                // previous, smaller square and new added cells.
                let mut total_power_level = 0;

                // Search all possible sizes from current top left corner.
                for n in 1..min(self.grid_size - i, self.grid_size - j) + 1 {
                    total_power_level += self.get(Position::new(j + n - 1, i + n - 1));
                    for k in 0..n - 1{
                        total_power_level += self.get(Position::new(j + n - 1, i + k));
                        total_power_level += self.get(Position::new(j + k, i + n - 1));
                    }

                    if total_power_level > top_square.power_level {
                        top_square = Square::new(Position::new(j, i), n, total_power_level);
                    }
                }
            }
        }

        top_square
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID_SIZE: usize = 300;
    const SQUARE_SIZE: usize = 3;

    #[test]
    fn test_power_level_122_79_57() {
        let fuel_cell_position = Position::new(122, 79);
        let serial_number = 57u32;

        assert_eq!(-5, Grid::get_power_level(fuel_cell_position, serial_number));
    }

    #[test]
    fn test_power_level_217_196_39() {
        let fuel_cell_position = Position::new(217, 196);
        let serial_number = 39u32;

        assert_eq!(0, Grid::get_power_level(fuel_cell_position, serial_number));
    }

    #[test]
    fn test_power_level_191_153_71() {
        let fuel_cell_position = Position::new(101, 153);
        let serial_number = 71u32;

        assert_eq!(4, Grid::get_power_level(fuel_cell_position, serial_number));
    }

    #[test]
    fn test_grid_power_level_122_79_57() {
        let fuel_cell_position = Position::new(122, 79);
        let serial_number = 57;
        let grid = Grid::new(serial_number, GRID_SIZE);

        assert_eq!(-5, grid.get(fuel_cell_position));
    }

    #[test]
    fn test_grid_power_level_217_196_39() {
        let fuel_cell_position = Position::new(217, 196);
        let serial_number = 39;
        let grid = Grid::new(serial_number, GRID_SIZE);

        assert_eq!(0, grid.get(fuel_cell_position));
    }

    #[test]
    fn test_grid_power_level_191_153_71() {
        let fuel_cell_position = Position::new(101, 153);
        let serial_number = 71;
        let grid = Grid::new(serial_number, GRID_SIZE);

        assert_eq!(4, grid.get(fuel_cell_position));
    }

    #[test]
    fn test_square_power_level() {
        let square_top_left = Position::new(33, 45);
        let serial_number = 18;
        let grid = Grid::new(serial_number, GRID_SIZE);

        assert_eq!(29, grid.get_square_power_level(square_top_left, SQUARE_SIZE));
    }

    #[test]
    fn test_largest_power_level_square() {
        let square_top_left = Position::new(33, 45);
        let serial_number = 18;
        let grid = Grid::new(serial_number, GRID_SIZE);
        let square = grid.square_with_largest_power(SQUARE_SIZE);

        assert_eq!(29, square.power_level);
        assert_eq!(square_top_left, square.top_left);
    }

    #[test]
    fn test_largest_power() {
        let square_with_largest_power = Square::new(
            Position::new(90, 269),
            16,
            113,
        );
        let serial_number = 18;
        let grid = Grid::new(serial_number, GRID_SIZE);
        let square = grid.largest_power();

        assert_eq!(square_with_largest_power, square);
    }
}