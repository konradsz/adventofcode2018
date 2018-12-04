use std::collections::HashMap;
use std::fs;

fn parse_minutes(input: &str) -> usize {
    let time = &input.split(']').next().unwrap();
    let len = time.len();
    let minutes = &time[len - 2..len];
    minutes.parse().unwrap()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let records: Vec<&str> = content.lines().collect();

    let mut shift_indices = Vec::new();
    for (index, entry) in records.iter().enumerate() {
        if entry.contains("begins shift") {
            shift_indices.push(index);
        }
    }

    let mut guards_sleep_time = HashMap::new();
    for index in shift_indices {
        let id: usize = records[index]
            .split_whitespace()
            .filter(|word| word.contains('#'))
            .next()
            .unwrap()
            .trim_matches('#')
            .parse()
            .unwrap();
        let entry = guards_sleep_time.entry(id).or_insert(vec![0; 60]);
        let mut start_time = 0;
        for i in index + 1..records.len() {
            if records[i].contains("falls asleep") {
                start_time = parse_minutes(records[i]);
            } else if records[i].contains("wakes up") {
                for i in start_time..parse_minutes(records[i]) {
                    entry[i] += 1;
                }
            } else {
                break;
            }
        }
    }

    let guard_id = guards_sleep_time
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.iter().sum::<usize>().cmp(&v2.iter().sum::<usize>()))
        .unwrap()
        .0;

    let most_appropriate_minute = guards_sleep_time
        .get(&guard_id)
        .unwrap()
        .iter()
        .enumerate()
        .max_by(|(_, i), (_, j)| i.cmp(j))
        .unwrap()
        .0;
    println!("part 1: {}", guard_id * most_appropriate_minute);

    let mut most_frequent_minute = 0;
    let mut most_frequent_minute_index = 0;
    let mut guard_id = 0;
    for (id, minutes) in guards_sleep_time.iter() {
        let i = minutes
            .iter()
            .enumerate()
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        if i.1 > &most_frequent_minute {
            most_frequent_minute = *i.1;
            most_frequent_minute_index = i.0;
            guard_id = *id;
        }
    }
    println!("part 2: {}", guard_id * most_frequent_minute_index);
}
