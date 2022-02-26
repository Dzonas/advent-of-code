mod lib;

use lib::Device;

fn main() {
    let device = Device::from_file("input");

    //println!("{:?}", device.frequencies);
    println!("Resulting frequency: {}", device.get_resulting_frequency());
    println!("Frequency reached twice: {}", device.get_reached_twice());
}
