mod lib;

use lib::Warehouse;

fn main() {
    let warehouse = Warehouse::from_file("input");
    let checksum = warehouse.get_checksum();

    println!("Checksum: {}", checksum);
    println!("Common letters: {}", warehouse.get_common_letters());
}
