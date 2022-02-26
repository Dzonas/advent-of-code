mod lib;

use lib::{Claim, Fabric, WIDTH, HEIGHT};

fn main() {
    let input = include_str!("../input");
    let claims = Claim::parse_input(input);
    let fabric = Fabric::new(WIDTH, HEIGHT, claims);

    println!("Overlap by at least 2: {}", fabric.overlap_size(2));
    println!("Not overlapping claim id: {}", fabric.get_not_overlapping_claim().unwrap().id());
    //println!("No overlap: {}", fabric.overlap_size(2));
}

