use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

static N_APPEARANCES: [u32; 2] = [2, 3];

pub struct Warehouse {
    boxes_id: Vec<String>
}

impl Warehouse {
    ///
    /// Reads file from path and creates Warehouse based on it.
    ///
    pub fn from_file(path: &'static str) -> Warehouse {
        let text = Warehouse::load_to_string(path); // Get original text from file
        let boxes_id = Warehouse::parse_string(text); // Extract id of the boxes

        Warehouse { boxes_id }
    }

    ///
    /// Reads whole file to a String.
    ///
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

    ///
    /// String parser.
    /// Collects each line to Vec<String>
    ///
    fn parse_string(text: String) -> Vec<String> {
        text.lines().map(|el| el.to_owned()).collect()
    }

    ///
    /// Calculates checksum of the warehouse based on the boxes id.
    ///
    pub fn get_checksum(&self) -> u32 {
        let mut occurences: HashMap<u32, u32> = HashMap::new();

        for id in &self.boxes_id {
            let id_histogram = Warehouse::get_word_histogram(id);

            for count in N_APPEARANCES.iter() {
                if id_histogram.values().any(|el| *el == *count) {
                    *occurences.entry(*count).or_insert(0) += 1
                }
            }
        }

        occurences.values().fold(1, |acc, x| acc * x)
    }

    ///
    /// Creates histogram of the word.
    /// Returns HashMap where K is a letter and V is a number it's uses in the word.
    ///
    fn get_word_histogram(word: &str) -> HashMap<char, u32> {
        let mut histogram = HashMap::new();

        // For each char in word add 1 to it's value in the histogram.
        // If entry doesn't exist, create it and set it to 0.
        word.chars().for_each(
            |letter| *histogram.entry(letter).or_insert(0) += 1
        );

        histogram
    }

    ///
    /// Returns Vec of words that differ by n characters.
    ///
    fn get_differing_by_n(words: &Vec<String>, n: u32) -> Vec<String> {
        let mut differing_by_one: Vec<String> = Vec::new();

        let mut i = 0;

        for first in &words[..] {
            for second in &words[i+1..] {
                let difference = Warehouse::get_difference(first, second);

                if difference == n {
                    if !differing_by_one.contains(&first) {
                        differing_by_one.push(first.clone());
                    }

                    if !differing_by_one.contains(second) {
                        differing_by_one.push(second.clone());
                    }
                }
            }

            i += 1;
        }

        differing_by_one
    }

    ///
    /// Returns a new String that is created by removing differing letters
    /// between two words.
    ///
    fn get_common(word1: &String, word2: &String) -> String {
        if word1.len() != word2.len() {
            panic!("Both words must be of the same length!");
        }

        let mut common_letters = String::new();

        let word_chars = word1.chars().zip(word2.chars());

        for (letter1, letter2) in word_chars {
            if letter1 == letter2 {
                common_letters.push(letter1);
            }
        }

        common_letters
    }

    ///
    /// Calculates common letters between boxes id
    /// that have 1 letter difference between them.
    ///
    pub fn get_common_letters(&self) -> String {
        let differing_by_one = Warehouse::get_differing_by_n(&self.boxes_id, 1);

        Warehouse::get_common(&differing_by_one[0], &differing_by_one[1])
    }

    ///
    /// Checks difference between two words.
    /// Returns number of places where two letter differ in those two words.
    ///
    fn get_difference(word1: &str, word2: &str) -> u32 {
        if word1.len() != word2.len() {
            panic!("Both words must be of the same length!");
        }

        let word_chars = word1.chars().zip(word2.chars());
        let mut difference = 0;

        for (letter1, letter2) in word_chars {
            if letter1 != letter2 {
                difference += 1;
            }
        }

        difference
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_difference() {
        assert_eq!(2, Warehouse::get_difference("abcde", "axcye"));
        assert_eq!(1, Warehouse::get_difference("fghij", "fguij"));
        assert_eq!(8, Warehouse::get_difference("abcdefgh", "hgfedcba"));
    }

    #[test]
    fn test_word_histogram() {
        let mut histogram: HashMap<char, u32> = HashMap::new();
        histogram.insert('b', 3);
        histogram.insert('a', 2);
        histogram.insert('c', 1);

        assert_eq!(histogram, Warehouse::get_word_histogram("bababc"));
    }
}
