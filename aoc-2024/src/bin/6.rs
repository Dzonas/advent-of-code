use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/6.txt");

    println!("{}", solve_part_1(input));
    println!("{}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut pos = (0, 0);
    let mut dir = (0, 0);

    for i in 0..height {
        for j in 0..width {
            let value = map[i as usize][j as usize];

            if value == '^' {
                pos = (i, j);
                dir = (-1, 0);
            } else if value == '>' {
                pos = (i, j);
                dir = (0, 1);
            } else if value == 'v' {
                pos = (i, j);
                dir = (1, 0);
            } else if value == '<' {
                pos = (i, j);
                dir = (0, -1);
            }
        }
    }

    let mut positions = HashSet::new();
    positions.insert(pos);
    loop {
        let (y, x) = (pos.0 + dir.0, pos.1 + dir.1);

        if y < 0 || y >= height || x < 0 || x >= width {
            break;
        }

        if map[y as usize][x as usize] == '#' {
            if dir == (-1, 0) {
                dir = (0, 1);
            } else if dir == (0, 1) {
                dir = (1, 0);
            } else if dir == (1, 0) {
                dir = (0, -1)
            } else if dir == (0, -1) {
                dir = (-1, 0);
            } else {
                panic!("invalid dir");
            }
        } else {
            pos = (y, x);
            positions.insert((y, x));
        }
    }

    positions.len() as u32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (isize, isize),
    dir: (isize, isize),
}

impl State {
    fn next_pos(&self) -> (isize, isize) {
        (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1)
    }

    fn next_dir(&self) -> (isize, isize) {
        let dir;
        if self.dir == (-1, 0) {
            dir = (0, 1);
        } else if self.dir == (0, 1) {
            dir = (1, 0);
        } else if self.dir == (1, 0) {
            dir = (0, -1);
        } else if self.dir == (0, -1) {
            dir = (-1, 0);
        } else {
            panic!("invalid dir");
        }

        dir
    }
}

fn solve_part_2(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut pos = (0, 0);
    let mut dir = (0, 0);

    for i in 0..height {
        for j in 0..width {
            let value = map[i as usize][j as usize];

            if value == '^' {
                pos = (i, j);
                dir = (-1, 0);
            } else if value == '>' {
                pos = (i, j);
                dir = (0, 1);
            } else if value == 'v' {
                pos = (i, j);
                dir = (1, 0);
            } else if value == '<' {
                pos = (i, j);
                dir = (0, -1);
            }
        }
    }

    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            let cell = map[i as usize][j as usize];

            if cell != '.' {
                continue;
            }

            let mut new_map = map.clone();
            new_map[i as usize][j as usize] = '#';
            let mut state = State { pos, dir };

            let mut positions: HashSet<State> = HashSet::new();
            loop {
                let (y, x) = state.next_pos();

                if y < 0 || y >= height || x < 0 || x >= width {
                    break;
                } else {
                    let new_state = if new_map[y as usize][x as usize] == '#' {
                        let new_dir = state.next_dir();
                        let new_pos = state.pos;

                        State {
                            pos: new_pos,
                            dir: new_dir,
                        }
                    } else {
                        let new_dir = state.dir;
                        let new_pos = state.next_pos();

                        State {
                            pos: new_pos,
                            dir: new_dir,
                        }
                    };

                    if positions.contains(&new_state) {
                        sum += 1;
                        break;
                    } else {
                        state = new_state;
                        positions.insert(state);
                    }
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "....#.....\n\
				 .........#\n\
				 ..........\n\
				 ..#.......\n\
				 .......#..\n\
				 ..........\n\
				 .#..^.....\n\
				 ........#.\n\
				 #.........\n\
				 ......#...";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(INPUT);

        assert_eq!(result, 41);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(INPUT);

        assert_eq!(result, 6);
    }
}
