use std::collections::{HashSet, HashMap};

pub struct Polymer {
    pub structure: String
}

impl Polymer {
    pub fn new(original_structure: &str) -> Polymer {
        let structure = String::from(original_structure);

        Polymer { structure }
    }

    pub fn trigger(&self) -> String {
        Polymer::react(&self.structure)
    }

    pub fn triggerv2(&self) -> usize {
        let mut agents = HashSet::new();

        for unit in self.structure.chars() {
            agents.insert(unit.to_lowercase().next().unwrap());
        }

        let mut results = HashMap::new();

        for agent in agents {
            println!("{}", agent);
            let mut new_structure = self.structure.clone();
            new_structure = Polymer::remove_same_type(new_structure, agent);
            new_structure = Polymer::react(&new_structure);

            results.insert(agent, new_structure.len());
        }

        let (_, &smallest_structure_length) = results.iter().min_by_key(|&(&k, &v)| v).unwrap();

        smallest_structure_length
    }

    fn react(structure: &String) -> String {
        let mut change = true;
        let mut current_structure = structure.clone();

        while change {
            change = false;

            let mut p1 = 0;
            let mut p2 = 1;
            let mut new_structure = String::new();
            let polymer_units: Vec<char> = current_structure.chars().collect();

            while p2 < polymer_units.len() {
                let a = polymer_units[p1];
                let b = polymer_units[p2];

                if Polymer::opposite_polarity(&a, &b) {
                    change = true;
                    p1 += 2;
                    p2 += 2;
                } else {
                    new_structure.push(a);
                    p1 += 1;
                    p2 += 1;
                }
            }

            if p1 == polymer_units.len() - 1 {
                new_structure.push(polymer_units[p1]);
            }

            current_structure = new_structure;
        }

        current_structure
    }

    fn remove_same_type(mut structure: String, unit_type: char) -> String {
        structure.retain(|c| c != unit_type && c != unit_type.to_uppercase().next().unwrap());
        structure
    }

    fn same_type(a: &char, b: &char) -> bool {
        a.to_lowercase().next().unwrap() == b.to_lowercase().next().unwrap()
    }

    fn opposite_polarity(a: &char, b: &char) -> bool {
        Polymer::same_type(a, b) && a != b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_type() {
        let a = 'A';
        let b = 'a';
        assert_eq!(true, Polymer::same_type(&a, &b));

        let a = 'a';
        let b = 'A';
        assert_eq!(true, Polymer::same_type(&a, &b));

        let a = 'b';
        let b = 'A';
        assert_eq!(false, Polymer::same_type(&a, &b));
    }

    #[test]
    fn test_opposite_polarity() {
        let a = 'A';
        let b = 'a';
        assert_eq!(true, Polymer::opposite_polarity(&a, &b));

        let a = 'a';
        let b = 'A';
        assert_eq!(true, Polymer::opposite_polarity(&a, &b));

        let a = 'a';
        let b = 'a';
        assert_eq!(false, Polymer::opposite_polarity(&a, &b));

        let a = 'A';
        let b = 'A';
        assert_eq!(false, Polymer::opposite_polarity(&a, &b));

        let a = 'b';
        let b = 'a';
        assert_eq!(false, Polymer::opposite_polarity(&a, &b));
    }

    #[test]
    fn test_trigger() {
        let input = include_str!("../test");
        let mut polymer = Polymer::new(input);
        let new_structure = polymer.trigger();

        assert_eq!("dabCBAcaDA", &new_structure);
    }

    #[test]
    fn test_triggerv2() {
        let input = include_str!("../test");
        let mut polymer = Polymer::new(input);
        let new_structure_length = polymer.triggerv2();

        assert_eq!(4, new_structure_length);
    }
}