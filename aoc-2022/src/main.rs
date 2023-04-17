mod year_2022;

use std::env;
use std::io;
use std::io::Read;

fn main() {
    let mut args = env::args();

    let _ = args.next();
    let year: u32 = args.next().unwrap().parse().unwrap();
    let day: u32 = args.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let (part_1_solution, part_2_solution) = match (year, day) {
        (2022, 1) => {
            use year_2022::day_01::{solve_part_1, solve_part_2};

            let part_1_solution = solve_part_1(&input);
            let part_2_solution = solve_part_2(&input);

            (part_1_solution, part_2_solution)
        }
        (2022, 2) => {
            use year_2022::day_02::{solve_part_1, solve_part_2};

            let part_1_solution = solve_part_1(&input);
            let part_2_solution = solve_part_2(&input);

            (part_1_solution, part_2_solution)
        }
        (2022, 3) => {
            use year_2022::day_03::{solve_part_1, solve_part_2};

            let part_1_solution = solve_part_1(&input);
            let part_2_solution = solve_part_2(&input);

            (part_1_solution, part_2_solution)
        }
        (2022, 4) => {
            use year_2022::day_04::{solve_part_1, solve_part_2};

            let part_1_solution = solve_part_1(&input);
            let part_2_solution = solve_part_2(&input);

            (part_1_solution, part_2_solution)
        }
        _ => panic!("Not solved yet"),
    };

    println!("{}", part_1_solution);
    println!("{}", part_2_solution);
}
