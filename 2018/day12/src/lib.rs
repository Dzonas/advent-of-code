use std::collections::{HashMap, VecDeque};

const DEPTH: usize = 5;

#[derive(Debug)]
pub struct Pots {
    state: VecDeque<char>,
    rules: HashMap<Vec<char>, char>,
    starting_index: usize
}

impl Pots {
    pub fn new(state: VecDeque<char>, rules: HashMap<Vec<char>, char>) -> Pots {
        let mut pots = Pots { state, rules, starting_index: 0 };
        pots.extend_ends();

        pots
    }

    ///
    /// Extends beginning of the state so that in the front there is DEPTH pots with no plants.
    ///
    fn extend_front(&mut self) {
        let mut counter = 0;

        for i in 0..DEPTH {
            if self.state[i] == '.' {
                counter += 1;
            } else {
                break;
            }
        }

        for _ in 0..DEPTH - counter {
            self.state.push_front('.');
            self.starting_index += 1;
        }
    }

    ///
    /// Extends end of the state so that on the end there is DEPTH pots with no plants.
    ///
    fn extend_back(&mut self) {
        let mut counter = 0;

        for i in (self.state.len() - DEPTH..self.state.len()).rev() {
            if self.state[i] == '.' {
                counter += 1;
            } else {
                break;
            }
        }

        for _ in 0..DEPTH - counter {
            self.state.push_back('.');
        }
    }

    ///
    /// Extends current state so that on the both ends there is five pots with no plants.
    /// Required for such rule.
    ///
    fn extend_ends(&mut self) {
        self.extend_front();
        self.extend_back();
    }

    ///
    /// Returns window containing elements [n-2, n-1, n, n+1, n+2]
    ///
    fn get(&self, n: usize) -> Vec<char> {
        let mut window = Vec::with_capacity(5);
        for i in n-2..=n+2 {
            window.push(*self.state.get(i).unwrap());
        }

        window
    }

    ///
    /// Updates pots according to the rules.
    /// Extends both sides of the current state to accommodate plant growth.
    ///
    fn tick(&mut self) {
        let mut lol: VecDeque<char> = self.state.clone();

        for i in 2..self.state.len() - 3 {
            let window = self.get(i);
            lol[i] = *self.rules.get(&window).unwrap();
        }

        self.state = lol;
        self.extend_ends();
    }

    ///
    /// Ticks simulation "generation" times
    ///
    pub fn simulate(&mut self, generations: usize) {
        for _ in 0..generations {
            self.tick();
        }
    }

    ///
    /// Calculates value of the whole plantation.
    /// Each plant has a value equal to it's relative index.
    /// No plant equals to 0.
    ///
    pub fn get_value(&self) -> i32 {
        let mut counter = 0;
        for (i, v) in self.state.iter().enumerate() {
            if *v == '#' {
                counter += i as i32 - self.starting_index as i32;
            }
        }

        counter
    }
}

fn parse_rules(lines: &VecDeque<&str>) -> HashMap<Vec<char>, char> {
    let mut rules = HashMap::new();

    for line in lines {
        let wtf: Vec<&str> = line.split(" => ").collect();
        let rule: Vec<char> = wtf[0].chars().collect();
        let result = wtf[1].chars().next().unwrap();

        rules.insert(rule, result);
    }

    rules
}

fn parse_initial_state(initial_state_line: &str) -> VecDeque<char> {
    initial_state_line[15..].chars().collect()
}

pub fn parse(text: &str) -> Pots {
    let mut lines: VecDeque<&str> = text.lines().collect();

    let state = parse_initial_state(lines.pop_front().unwrap()); // Initial state of the pots
    lines.pop_front(); // Next line is empty
    let rules = parse_rules(&mut lines); // Rules of the plant growth

    Pots::new(state, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../test");

    #[test]
    fn test_parse() {
        let mut pots = parse(INPUT);
        pots.simulate(20);

        assert_eq!(325, pots.get_value());
    }
}