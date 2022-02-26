mod lib;

use crate::lib::{parse, Node};

fn main() {
    let input = include_str!("../input");
    let data = parse(input);
    let node = Node::from_data(data);

    println!("Sum of all metadata entries is: {}", node.get_metadata_sum());
    println!("Value of the root node is: {}", node.get_value());
}



