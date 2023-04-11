mod year_2022;

use year_2022::day_01::{solve_part_1, solve_part_2};

fn main() {
    let input_2022_01 = include_str!("../input/2022/01.txt");

    let part_1_solution = solve_part_1(input_2022_01);
    let part_2_solution = solve_part_2(input_2022_01);

    println!("{}", part_1_solution);
    println!("{}", part_2_solution);
}
