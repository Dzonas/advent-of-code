mod lib;

use crate::lib::parse;

fn main() {
    let input = include_str!("../input");
    let mut sky = parse(input);

    let t = sky.tick_until_smallest_area();
    println!("{:?}", sky);
    println!("Time: {}", t);
}
