use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input");
    let star_map = parse(input);

    let total_number_of_orbits = count_number_of_orbits(&star_map);
    let orbital_transfers = count_orbital_transfers(&star_map, "YOU", "SAN");

    println!("Part 1: {}", total_number_of_orbits);
    println!("Part 2: {}", orbital_transfers);
}

/// Counts number of orbital transfers needed between "from" and "to".
fn count_orbital_transfers(star_map: &HashMap<&str, &str>, from: &str, to: &str) -> usize {
    let mut current = star_map[from];
    let mut road_to_com = HashSet::new();
    road_to_com.insert(current);

    while let Some(&next) = star_map.get(current) {
        road_to_com.insert(next);
        current = next;
    }

    let mut common = star_map[to];

    loop {
        if road_to_com.contains(&common) {
            break;
        } else {
            common = star_map[common];
        }
    }

    distance(&star_map, "YOU", common) + distance(&star_map, "SAN", common)
}

/// Counts how many orbital transfers are needed by two objects.
/// "from" must orbit "to" at least indirectly.
fn distance(star_map: &HashMap<&str, &str>, from: &str, to: &str) -> usize {
    let mut dist = 0;
    let mut current = from;

    while let Some(&next) = star_map.get(current) {
        if next == to {
            return dist;
        } else {
            current = next;
            dist += 1;
        }
    }

    panic!("from doesn't orbit to");
}

/// Counts total number of direct and indirect orbits for each planet.
fn count_number_of_orbits(star_map: &HashMap<&str, &str>) -> usize {
    let mut counter = 0;

    for &planet in star_map.keys() {
        counter += number_of_orbits(&star_map, planet);
    }

    counter
}

/// Counts how many objects "planet" orbits (directly + indirectly)
fn number_of_orbits(star_map: &HashMap<&str, &str>, planet: &str) -> usize {
    if planet == "COM" {
        return 0;
    } else {
        return 1 + number_of_orbits(&star_map, star_map[planet]);
    }
}

fn parse(input: &str) -> HashMap<&str, &str> {
    let mut orbits = HashMap::new();
    for line in input.lines() {
        let data: Vec<&str> = line.split(')').collect();
        orbits.insert(data[1], data[0]);
    }
    orbits
}
