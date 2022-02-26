fn main() {
    let input = include_str!("../input");
    let data: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    // Part 1
    let fuel: i32 = data.iter().map(|v| (v / 3) - 2).sum();

    println!("Part 1: {}", fuel);

    // Part 2
    //std::iter::successors(Some(0), )
    // let mut total_fuel = 0;

    // for v in data {
    //     let mut v = (v / 3) - 2;

    //     while v > 0 {
    //         total_fuel += v;
    //         v = (v / 3) - 2;
    //     }
    // }

    println!("Part 2: {}", total_fuel);
}
