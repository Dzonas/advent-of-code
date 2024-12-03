use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/3.txt");

    println!("{}", solve_part_1(input));
    println!("{}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("valid regex must be provided");

    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let x: u32 = a.parse().unwrap();
        let y: u32 = b.parse().unwrap();

        sum += x * y;
    }

    sum
}

fn solve_part_2(input: &str) -> u32 {
    let re = Regex::new(r"(don't\(\))|(do\(\))|(mul\((\d+),(\d+)\))")
        .expect("valid regex must be provided");

    let mut sum = 0;
    let mut enabled = true;
    for x in re.captures_iter(input) {
        if x.get(1).is_some() {
            enabled = false;
        } else if x.get(2).is_some() {
            enabled = true;
        } else if enabled && x.get(3).is_some() {
            let a: u32 = x.get(4).unwrap().as_str().parse().unwrap();
            let b: u32 = x.get(5).unwrap().as_str().parse().unwrap();
            sum += a * b;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = solve_part_1(input);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_solve_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = solve_part_2(input);

        assert_eq!(result, 48);
    }
}
