use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub struct Device {
    frequencies: Vec<i32>
}

impl Device {
    pub fn from_file(path: &'static str) -> Device {
        let text = Device::load_to_string(path); // Get original text from file
        let frequencies = Device::parse_string(text); // Extract integers

        Device { frequencies }
    }

    fn load_to_string(path: &'static str) -> String {
        // Open file
        // If file not found - panic
        let mut f = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!(err)
        };

        // Read text to String
        // If reading goes wrong - panic
        let mut buffer = String::new();
        match f.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(err) => panic!(err)
        };

        buffer
    }

    fn parse_string(text: String) -> Vec<i32> {
        let mut frequencies = Vec::new();

        for line in text.lines() {
            let str_number = line.trim_end_matches(','); // Remove characters from the end of the line
            let number: i32 = match str_number.parse() {
                Ok(n) => n,
                Err(_) => panic!("Could not convert string to i32. File contains invalid formatting or characters.")
            };

            frequencies.push(number);
        }

        frequencies
    }

    pub fn get_resulting_frequency(&self) -> i32 {
        self.frequencies.iter().sum()
    }

    pub fn get_reached_twice(&self) -> i32 {
        let mut current_frequency = 0;
        let mut reached_frequencies = HashSet::new();
        //let mut reached_frequencies = vec![0];
        let frequencies = self.frequencies.iter().cycle();

        for f in frequencies {
            current_frequency += *f;

            if reached_frequencies.contains(&current_frequency) {
                break;
            } else {
                reached_frequencies.insert(current_frequency);
            }
        }

        current_frequency
    }
}