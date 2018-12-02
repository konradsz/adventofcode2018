use std::fs;

fn check_if_common(word_1: &str, word_2: &str) {
    let length = word_1.len();
    let common_letters = word_1
        .chars()
        .zip(word_2.chars())
        .filter(|(letter_1, letter_2)| letter_1 == letter_2)
        .count();

    if common_letters == length - 1 {
        let common_letters: String = word_1
            .chars()
            .zip(word_2.chars())
            .filter(|(letter_1, letter_2)| letter_1 == letter_2)
            .map(|(a, _)| a)
            .collect();
        println!("{}", common_letters);
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let entries: Vec<&str> = content.lines().collect();

    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            check_if_common(entries[i], entries[j]);
        }
    }
}
