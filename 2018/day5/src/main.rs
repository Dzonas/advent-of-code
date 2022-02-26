mod lib;

use crate::lib::Polymer;

fn main() {
    let input = include_str!("../input").trim();
    let mut polymer = Polymer::new(input);
    let new_structure = polymer.trigger();

    println!("New polymer length: {}", new_structure.len());
    let smallest_structure_length = polymer.triggerv2();
    println!("Smallest polymer length: {}", smallest_structure_length);
}
