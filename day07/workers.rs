use std::collections::HashMap;
use std::fs;

#[derive(Clone, PartialEq)]
enum WorkerState {
    Idle,
    Busy,
}

#[derive(Clone)]
struct Worker {
    state: WorkerState,
    step: char,
    time: u32,
}

fn get_next_step(requirements: &HashMap<char, Vec<char>>) -> Option<char> {
    let mut steps: Vec<char> = requirements
        .iter()
        .filter(|(_, requirement)| requirement.is_empty())
        .map(|(step, _)| *step)
        .collect();
    steps.sort();

    if steps.first().is_some() {
        Some(*steps.first().unwrap())
    } else {
        None
    }
}

fn mark_as_worked_on(requirement: char, requirements: &mut HashMap<char, Vec<char>>) {
    requirements.remove(&requirement);
}

fn mark_as_done(requirement: char, requirements: &mut HashMap<char, Vec<char>>) {
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

    const NUMBER_OF_WORKERS: usize = 5;
    let mut workers = vec![
        Worker {
            state: WorkerState::Idle,
            step: 0 as char,
            time: 0
        };
        NUMBER_OF_WORKERS
    ];

    let mut total_time = 0;
    while requirements.len() > 0 || workers
        .iter()
        .any(|worker| worker.state == WorkerState::Busy)
    {
        for worker in workers
            .iter_mut()
            .filter(|worker| worker.state == WorkerState::Busy)
        {
            worker.time -= 1;
            if worker.time == 0 {
                mark_as_done(worker.step, &mut requirements);
                worker.state = WorkerState::Idle;
            }
        }

        for worker in workers
            .iter_mut()
            .filter(|worker| worker.state == WorkerState::Idle)
        {
            let step = get_next_step(&requirements);
            if step.is_some() {
                worker.state = WorkerState::Busy;
                worker.step = step.unwrap();
                worker.time = step.unwrap() as u32 - 64 + 60;
                mark_as_worked_on(worker.step, &mut requirements);
            }
        }

        total_time += 1;
    }
    println!("{}", total_time - 1);
}
