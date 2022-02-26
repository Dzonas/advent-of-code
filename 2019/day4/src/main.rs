fn split_digits(mut n: u32) -> Vec<u32> {
    let mut digits = Vec::new();

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();

    digits
}

fn check_part1<'a>(mut digits: impl Iterator<Item = &'a u32>) -> bool {
    let mut prev = digits.next().unwrap();
    let mut stayed_same = false; // Check it at least once the value didnt increase

    for d in digits {
        if d < prev {
            return false;
        } else if d == prev {
            stayed_same = true;
        }

        prev = d;
    }

    stayed_same
}

fn check_part2<'a>(mut digits: impl Iterator<Item = &'a u32>) -> bool {
    let mut prev = digits.next().unwrap();
    let mut stayed_same = false; // Check it at least once the value didnt increase
    let mut counter = 0;

    for d in digits {
        if d < prev {
            return false;
        }

        if d == prev && !stayed_same {
            counter += 1;
        } else {
            if counter == 1 {
                stayed_same = true;
            } else {
                counter = 0;
            }
        }

        prev = d;
    }

    stayed_same | (counter == 1)
}

fn main() {
    let mut part1_counter = 0;
    let mut part2_counter = 0;

    for i in 307237..=769058 {
        let digits = split_digits(i);
        if check_part1(digits.iter()) {
            part1_counter += 1;
        }

        if check_part2(digits.iter()) {
            part2_counter += 1;
        }
    }

    println!("Part 1: {}", part1_counter);
    println!("Part 2: {}", part2_counter);
}
