use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/5.txt");

    println!("{}", solve_part_1(input));
    println!("{}", solve_part_2(input));
}

struct Input {
    page_numbers_with_rules: HashSet<u32>,
    rules: HashSet<[u32; 2]>,
    updates: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let (page_numbers_with_rules, rules) =
        parse_rules(parts.next().expect("input must contain rules"));
    let updates = parse_updates(parts.next().expect("input must contain updates"));

    Input {
        page_numbers_with_rules,
        rules,
        updates,
    }
}

fn parse_rules(rules: &str) -> (HashSet<u32>, HashSet<[u32; 2]>) {
    let page_numbers_with_rules: HashSet<u32> = rules.lines().flat_map(to_rule).collect();
    let rules: HashSet<[u32; 2]> = rules.lines().map(to_rule).collect();

    (page_numbers_with_rules, rules)
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates.lines().map(to_page_numbers).collect()
}

fn to_rule(line: &str) -> [u32; 2] {
    let mut splits = line.split("|");
    let a = splits
        .next()
        .expect("must have first part of the rule")
        .parse()
        .expect("first part of the rule must be a number");
    let b = splits
        .next()
        .expect("must have second part of the rule")
        .parse()
        .expect("second part of the rule must be a number");

    [a, b]
}

fn to_page_numbers(update: &str) -> Vec<u32> {
    update
        .split(",")
        .map(|page_number| page_number.parse().expect("page number must be a number"))
        .collect()
}

fn solve_part_1(input: &str) -> u32 {
    let input = parse_input(input);

    let mut sum = 0;
    'outer: for update in input.updates {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                let a = update[i];
                let b = update[j];

                if input.page_numbers_with_rules.contains(&a)
                    && input.page_numbers_with_rules.contains(&b)
                    && input.rules.contains(&[b, a])
                {
                    continue 'outer;
                }
            }
        }

        sum += update[update.len() / 2];
    }

    sum
}

fn solve_part_2(input: &str) -> u32 {
    let mut input = parse_input(input);

    let mut sum = 0;
    for update in &mut input.updates {
        let mut incorrect = false;

        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                let a = update[i];
                let b = update[j];

                if input.page_numbers_with_rules.contains(&a)
                    && input.page_numbers_with_rules.contains(&b)
                    && input.rules.contains(&[b, a])
                {
                    incorrect = true;
                    update[i] = b;
                    update[j] = a;
                }
            }
        }
        if incorrect {
            sum += update[update.len() / 2];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "47|53\n\
				 97|13\n\
				 97|61\n\
				 97|47\n\
				 75|29\n\
				 61|13\n\
				 75|53\n\
				 29|13\n\
				 97|29\n\
				 53|29\n\
				 61|53\n\
				 97|53\n\
				 61|29\n\
				 47|13\n\
				 75|47\n\
				 97|75\n\
				 47|61\n\
				 75|61\n\
				 47|29\n\
				 75|13\n\
				 53|13\n\
				 \n\
				 75,47,61,53,29\n\
				 97,61,53,29,13\n\
				 75,29,13\n\
				 75,97,47,61,53\n\
				 61,13,29\n\
				 97,13,75,29,47";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(INPUT);

        assert_eq!(result, 143);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(INPUT);

        assert_eq!(result, 123);
    }
}
