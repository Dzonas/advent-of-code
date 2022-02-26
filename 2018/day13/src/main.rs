mod lib;

use crate::lib::parse;

fn main() {
    let input = include_str!("../input");
    let mut track_system = parse(input);
    let crash = track_system.simulate_until_crash();

    println!("Crash: {:?}", crash);

    track_system = parse(input);
    let last_cart_standing = track_system.simulate_until_last();

    println!("Last cart: {:?}", last_cart_standing);
}
