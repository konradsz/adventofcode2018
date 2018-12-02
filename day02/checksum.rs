use std::fs;
use std::collections::HashMap;

fn main() {
    let mut twice = 0;
    let mut thrice = 0;
    for line in fs::read_to_string("input").unwrap().lines() {
        let mut letters: HashMap<char, u32> = HashMap::new();
        for letter in line.chars() {
            *letters.entry(letter).or_insert(0) += 1;
        }

        if letters.values().any(|&value| value == 2) {
            twice += 1;
        }
        if letters.values().any(|&value| value == 3) {
            thrice += 1;
        }
    }

    println!("{}", twice * thrice)
}
