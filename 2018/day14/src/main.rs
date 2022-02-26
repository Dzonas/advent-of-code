struct Kitchen {
    elfs: Vec<usize>, // Elf's chosen recipes.
    recipes: Vec<u32> // Recipe's scores.
}

impl Kitchen {
    fn new(elfs: Vec<usize>, recipes: Vec<u32>) -> Self {
        Kitchen { elfs, recipes }
    }

    fn move_forward(&mut self, elf: usize, steps: usize) {
        assert!(elf < self.elfs.len());

        self.elfs[elf] = (self.elfs[elf] + steps) % self.recipes.len();
    }

    fn create_new_recipes(&self) -> Vec<u32> {
        let mut total_score = 0;

        for elf in self.elfs.iter() {
            total_score += self.recipes[*elf];
        }

        let new_recipies: Vec<_> = total_score.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();

        new_recipies
    }

    fn add_recipes(&mut self, mut new_recipes: Vec<u32>) {
        self.recipes.append(&mut new_recipes);
    }

    fn make_recipes(&mut self, n: usize) {
        while self.recipes.len() < n {
            let new_recipes = self.create_new_recipes();
            self.add_recipes(new_recipes);

            for i in 0..self.elfs.len() {
                self.move_forward(i, self.recipes[self.elfs[i]] as usize + 1);
            }
        }
    }

    fn left_of_sequence(&mut self, sequence: &[u32]) -> usize {
        loop {
            println!("{}", self.recipes.len());
            let old_len = self.recipes.len();
            self.make_recipes(self.recipes.len() + 1);
            let new_len = self.recipes.len();
            let diff = new_len - old_len;

            if self.recipes.len() < sequence.len() + diff {
                continue;
            }

            for i in 0..diff {
                let hmm = &self.recipes[self.recipes.len()-sequence.len()-i..self.recipes.len() - i];
                if hmm == sequence {
                    return self.recipes.len()-sequence.len()-i;
                }
            }
        }
    }
}

fn main() {
    let n = 323081;
    let elfs = vec![0, 1];
    let recipes = vec![3, 7];
    let mut kitchen = Kitchen::new(elfs.clone(), recipes.clone());
    kitchen.make_recipes(n + 10);

    let mut score = String::new();
    kitchen.recipes.iter().skip(n).take(10).for_each(|r| score.push_str(&r.to_string()));
    println!("Scores of 10 recipes immediately after {} recipes: {}.", n, score);

    let mut kitchen = Kitchen::new(elfs.clone(), recipes.clone());
    let n_recipes = kitchen.left_of_sequence(&[3, 2, 3, 0, 8, 1]);

    println!("Number of recipes to the left of the score sequence 323081: {}.", n_recipes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_recipe() {    
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let kitchen = Kitchen::new(elfs, recipes);

        let new_recipes = kitchen.create_new_recipes();

        assert_eq!(vec![1, 0], new_recipes);
    }

    #[test]
    fn test_move_forward() {
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);

        let new_recipes = kitchen.create_new_recipes();
        kitchen.add_recipes(new_recipes);

        for i in 0..kitchen.elfs.len() {
            kitchen.move_forward(i, kitchen.recipes[kitchen.elfs[i]] as usize + 1);
        }

        assert_eq!([0, 1], &kitchen.elfs[..]);

        let new_recipes = kitchen.create_new_recipes();
        kitchen.add_recipes(new_recipes);

        for i in 0..kitchen.elfs.len() {
            kitchen.move_forward(i, kitchen.recipes[kitchen.elfs[i]] as usize + 1);
        }

        assert_eq!([4, 3], &kitchen.elfs[..]);
    }

    #[test]
    fn test_make_recipes() {
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);
        kitchen.make_recipes(19);

        assert_eq!([3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9], &kitchen.recipes[..]);
    }

    #[test]
    fn test_after_9() {
        let n = 9;
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);
        kitchen.make_recipes(n + 10);

        let mut score = String::new();
        kitchen.recipes.iter().skip(n).take(10).for_each(|r| score.push_str(&r.to_string()));
        assert_eq!("5158916779", score);
    }

    #[test]
    fn test_after_5() {
        let n = 5;
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);
        kitchen.make_recipes(n + 10);

        let mut score = String::new();
        kitchen.recipes.iter().skip(n).take(10).for_each(|r| score.push_str(&r.to_string()));
        assert_eq!("0124515891", score);
    }

    #[test]
    fn test_after_18() {
        let n = 18;
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);
        kitchen.make_recipes(n + 10);

        let mut score = String::new();
        kitchen.recipes.iter().skip(n).take(10).for_each(|r| score.push_str(&r.to_string()));
        assert_eq!("9251071085", score);
    }

    #[test]
    fn test_after_2018() {
        let n = 2018;
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);
        kitchen.make_recipes(n + 10);

        let mut score = String::new();
        kitchen.recipes.iter().skip(n).take(10).for_each(|r| score.push_str(&r.to_string()));
        assert_eq!("5941429882", score);
    }

    #[test]
    fn left_of_sequence_51589() {
        let sequence = [5, 1, 5, 8, 9];
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);

        let n_recipes = kitchen.left_of_sequence(&sequence);

        assert_eq!(9, n_recipes);
    }

    #[test]
    fn left_of_sequence_01245() {
        let sequence = [0, 1, 2, 4, 5];
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);

        let n_recipes = kitchen.left_of_sequence(&sequence);

        assert_eq!(5, n_recipes);
    }

    #[test]
    fn left_of_sequence_92510() {
        let sequence = [9, 2, 5, 1, 0];
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);

        let n_recipes = kitchen.left_of_sequence(&sequence);

        assert_eq!(18, n_recipes);
    }

    #[test]
    fn left_of_sequence_59414() {
        let sequence = [5, 9, 4, 1, 4];
        let elfs = vec![0, 1];
        let recipes = vec![3, 7];
        let mut kitchen = Kitchen::new(elfs, recipes);

        let n_recipes = kitchen.left_of_sequence(&sequence);

        assert_eq!(2018, n_recipes);
    }
}
