mod lib;

use crate::lib::{Process, parse};

fn main() {
    let input = include_str!("../input");
    let (steps1, requirements1) = parse(input);
    let (steps2, requirements2) = (steps1.clone(), requirements1.clone());
    let mut process1 = Process::new(steps1, requirements1);
    let mut process2 = Process::new(steps2, requirements2);

    let ordering: String = process1.get_ordering().into_iter().collect();
    println!("Ordering: {:?}", ordering);

    let n_workers = 5;
    let time = process2.complete_time(n_workers, &work_time);
    println!("Time to complete task with {} workers: {}", n_workers, time);
}

fn work_time(step: char) -> u8 {
    step as u8 - 4
}
