mod lib;

use crate::lib::{parse};

fn main() {
    let input = include_str!("../input");
    let mut pots = parse(input);
    pots.simulate(20);
    println!("Value after 20 generations: {}", pots.get_value());

    pots.simulate(980);
    let x1 = 1000i64;
    let y1 = pots.get_value() as i64;
    pots.simulate(1);
    let x2 = 1001i64;
    let y2 = pots.get_value() as i64;

    let a = (y1 - y2) / (x1 - x2);
    let b = (y1 * x2 - y2 * x1) / (x2 - x1);

    let x = 50_000_000_000;
    let y = a * x + b;

    println!("Value after 50 000 000 000 generations: {}", y);
}
