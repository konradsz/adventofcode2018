use std::fs;
use std::collections::HashSet;

fn main() {
    let frequencies: Vec<i32> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    let sum: i32 = frequencies.iter().sum();
    println!("Sum: {}", sum);

    let mut reached_frequencies = HashSet::new();
    reached_frequencies.insert(0);
    let mut sum = 0;
    for frequency in frequencies.iter().cycle() {
        sum += *frequency;

        if reached_frequencies.contains(&sum) {
            println!("First frequency reached twice: {}", sum);
            break;
        }
        reached_frequencies.insert(sum);
    }
}
