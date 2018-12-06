use std::fs;

fn part_1(polymer: &str) {
    let mut polymer: Vec<char> = polymer.chars().collect();
    let mut finished = false;

    while !finished {
        let mut new_polymer = Vec::new();
        let mut i = 0;
        finished = true;

        while i < polymer.len() {
            if i == polymer.len() - 1 {
                new_polymer.push(polymer[i]);
                break;
            }

            if (polymer[i] as i32 - polymer[i + 1] as i32).abs() == 32 {
                i += 2;
                finished = false;
            } else {
                new_polymer.push(polymer[i]);
                i += 1;
            }
        }
        polymer = new_polymer;
    }

    println!("Part 1: {}", polymer.len());
}

fn part_2(polymer: &str) {
    let mut lengths = Vec::new();
    for unit in 'a' as u8..'z' as u8 {
        let mut polymer: Vec<char> = polymer
            .chars()
            .filter(|&c| c.to_lowercase().next().unwrap() != unit as char)
            .collect();
        let mut finished = false;

        while !finished {
            let mut new_polymer = Vec::new();
            let mut i = 0;
            finished = true;

            while i < polymer.len() {
                if i == polymer.len() - 1 {
                    new_polymer.push(polymer[i]);
                    break;
                }

                if (polymer[i] as i32 - polymer[i + 1] as i32).abs() == 32 {
                    i += 2;
                    finished = false;
                } else {
                    new_polymer.push(polymer[i]);
                    i += 1;
                }
            }
            polymer = new_polymer;
        }

        lengths.push(polymer.len());
    }

    println!("Part 2: {}", lengths.iter().min().unwrap());
}

fn main() {
    let polymer = fs::read_to_string("input").unwrap();
    let polymer = polymer.trim();

    part_1(&polymer);
    part_2(&polymer);
}
