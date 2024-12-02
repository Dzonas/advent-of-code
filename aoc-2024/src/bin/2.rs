fn main() {
    let input = include_str!("../../inputs/2.txt");
    let parsed_input = parse(input);

    println!("{}", solve_part_1(&parsed_input));
    println!("{}", solve_part_2(&parsed_input));
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut v = Vec::new();
    for line in input.lines() {
        let mut inner_v = Vec::new();
        for c in line.split(" ") {
            let n: u32 = c.parse().expect("levels should be numbers");
            inner_v.push(n);
        }
        v.push(inner_v);
    }

    v
}

fn solve_part_1(input: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for report in input {
        let v = gen_level_diff(report);
        if is_safe(&v) {
            sum += 1;
        }
    }

    sum
}

fn solve_part_2(input: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for report in input {
        // Create a Vec that contains the original report as well as new reports
        // created by removing one element from the original report
        let mut subreports = Vec::with_capacity(report.len() + 1);
        subreports.push(report.clone());
        for i in 0..report.len() {
            let mut subreport = Vec::with_capacity(report.len() - 1);
            for (j, n) in report.iter().enumerate() {
                if i != j {
                    subreport.push(*n);
                }
            }
            subreports.push(subreport);
        }

        // Calculate difference between subsequent levels
        let subreport_level_diffs: Vec<Vec<i32>> = subreports
            .iter()
            .map(|subreport| gen_level_diff(subreport))
            .collect();

        // If any of the subreports are safe then the report is safe
        if subreport_level_diffs.iter().any(|a| is_safe(a)) {
            sum += 1;
        }
    }

    sum
}

fn is_safe(report: &[i32]) -> bool {
    report.iter().all(|n| [-1, -2, -3].contains(n)) || report.iter().all(|n| [1, 2, 3].contains(n))
}

fn gen_level_diff(report: &[u32]) -> Vec<i32> {
    let mut v = Vec::with_capacity(report.len() - 1);
    for levels in report.windows(2) {
        let [a, b]: [_; 2] = levels.try_into().unwrap();
        v.push(a as i32 - b as i32);
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "7 6 4 2 1\n\
				 1 2 7 8 9\n\
				 9 7 6 2 1\n\
				 1 3 2 4 5\n\
				 8 6 4 4 1\n\
				 1 3 6 7 9";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(&parse(INPUT));

        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(&parse(INPUT));

        assert_eq!(result, 4);
    }
}
