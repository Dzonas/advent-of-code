use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    let input = include_str!("../input");
    let (min_dist, min_length) = solve(input);

    println!("Part 1: {}", min_dist);
    println!("Part 2: {}", min_length);
}

fn solve(input: &str) -> (i32, usize) {
    let (a, b) = parse_input(input);

    // Point to wire length map
    let a_points = generate_points(a);
    let b_points = generate_points(b);

    // Set of unique points
    let unique_a_points: HashSet<(i32, i32)> = HashSet::from_iter(a_points.keys().cloned());
    let unique_b_points: HashSet<(i32, i32)> = HashSet::from_iter(b_points.keys().cloned());

    // Get unqiue intersections between two wires
    let intersections = unique_a_points.intersection(&unique_b_points);

    // Get intersection closest to 0,0
    let min_dist = intersections
        .clone()
        .min_by_key(|p| manhattan_distance(p))
        .unwrap();

    // Get intersection with least total wire length
    let min_length = intersections
        .min_by_key(|k| a_points[k] + b_points[k])
        .unwrap();

    (
        manhattan_distance(min_dist),
        a_points[min_length] + b_points[min_length],
    )
}

fn manhattan_distance(p: &(i32, i32)) -> i32 {
    i32::abs(p.0) + i32::abs(p.1)
}

fn parse_input(input: &str) -> (Vec<(char, i32)>, Vec<(char, i32)>) {
    let mut iter = input.lines();
    let a = parse_line(iter.next().unwrap());
    let b = parse_line(iter.next().unwrap());

    (a, b)
}

fn parse_line(line: &str) -> Vec<(char, i32)> {
    line.split(',')
        .map(|v| {
            let (dir, len) = v.split_at(1);
            (dir.chars().next().unwrap(), len.parse::<i32>().unwrap())
        })
        .collect()
}

fn generate_points(lines: Vec<(char, i32)>) -> HashMap<(i32, i32), usize> {
    let mut points = HashMap::new();
    let (mut x, mut y) = (0, 0);
    let mut n = 0;
    for (direction, length) in lines {
        let (x_move, y_move);
        match direction {
            'L' => {
                x_move = -1;
                y_move = 0;
            }
            'D' => {
                x_move = 0;
                y_move = -1;
            }
            'U' => {
                x_move = 0;
                y_move = 1;
            }
            'R' => {
                x_move = 1;
                y_move = 0;
            }
            _ => panic!("unknown direction"),
        }

        for _ in 0..length {
            x += x_move;
            y += y_move;
            n += 1;
            points.insert((x, y), n);
        }
    }

    points
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const WIRES_1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
    const WIRES_2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const WIRES_3: &str =
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn wire_1() {
        let (min_dist, min_length) = solve(WIRES_1);

        assert_eq!(6, min_dist);
        assert_eq!(30, min_length);
    }

    #[test]
    fn wire_2() {
        let (min_dist, min_length) = solve(WIRES_2);

        assert_eq!(159, min_dist);
        assert_eq!(610, min_length);
    }

    #[test]
    fn wire_3() {
        let (min_dist, min_length) = solve(WIRES_3);

        assert_eq!(135, min_dist);
        assert_eq!(410, min_length);
    }
}
