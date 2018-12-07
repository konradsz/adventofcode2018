use std::collections::HashMap;
use std::fs;

fn get_next_task(requirements: &HashMap<char, Vec<char>>) -> char {
    let mut tasks: Vec<char> = requirements
        .iter()
        .filter(|(_, requirement)| requirement.is_empty())
        .map(|(task, _)| *task)
        .collect();
    tasks.sort();
    *tasks.first().unwrap()
}

fn remove_from_requirements(requirement: char, requirements: &mut HashMap<char, Vec<char>>) {
    requirements.remove(&requirement);
    requirements
        .values_mut()
        .filter(|value| value.contains(&requirement))
        .for_each(|value| {
            let index = value.iter().position(|r| *r == requirement).unwrap();
            value.remove(index);
        });
}

fn main() {
    let mut requirements = HashMap::new();

    for line in fs::read_to_string("input").unwrap().lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let first = words[1].chars().next().unwrap();
        let second = words[7].chars().next().unwrap();

        requirements.entry(first).or_insert(vec![]);
        let entry = requirements.entry(second).or_insert(vec![]);
        (*entry).push(first);
    }

    while requirements.len() > 0 {
        let task = get_next_task(&requirements);
        print!("{}", task);
        remove_from_requirements(task, &mut requirements);
    }
    println!("");
}
