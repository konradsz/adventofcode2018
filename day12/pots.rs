use std::collections::VecDeque;
use std::fs;

struct Rule {
    left_left: bool,
    left: bool,
    current: bool,
    right: bool,
    right_right: bool,
    new_generation: bool,
}

impl Rule {
    fn new(rule: &str) -> Rule {
        Rule {
            left_left: rule.chars().nth(0).unwrap() == '#',
            left: rule.chars().nth(1).unwrap() == '#',
            current: rule.chars().nth(2).unwrap() == '#',
            right: rule.chars().nth(3).unwrap() == '#',
            right_right: rule.chars().nth(4).unwrap() == '#',
            new_generation: rule.chars().nth(9).unwrap() == '#',
        }
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Rule) -> bool {
        self.left_left == other.left_left
            && self.left == other.left
            && self.current == other.current
            && self.right == other.right
            && self.right_right == other.right_right
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut lines = content.lines();
    let initial_state = lines.next().unwrap().replace("initial state: ", "");
    lines.next(); // ignore empty line

    let mut pots = VecDeque::new();

    initial_state.chars().for_each(|c| {
        if c == '#' {
            pots.push_back(true);
        } else {
            pots.push_back(false);
        }
    });

    let mut rules = Vec::new();
    lines.for_each(|line| rules.push(Rule::new(line)));

    let mut pushed_front = 0;
    for _ in 0..20 {
        while pots.iter().take_while(|pot| **pot == false).count() <= 4 {
            pots.push_front(false);
            pushed_front += 1;
        }

        while pots.iter().rev().take_while(|pot| **pot == false).count() <= 4 {
            pots.push_back(false);
        }

        let mut new_generation_pots = pots.clone();

        for i in 2..pots.len() - 2 {
            for rule in &rules {
                if *rule
                    == (Rule {
                        left_left: pots[i - 2],
                        left: pots[i - 1],
                        current: pots[i],
                        right: pots[i + 1],
                        right_right: pots[i + 2],
                        new_generation: Default::default(), // not taken into account in comparison
                    })
                {
                    new_generation_pots[i] = rule.new_generation;
                    break;
                } else {
                    new_generation_pots[i] = false;
                }
            }
        }

        pots = new_generation_pots;
    }

    let sum = pots
        .iter()
        .enumerate()
        .filter(|(_, pot)| **pot)
        .fold(0, |acc, (i, _)| acc + (i as i32 - pushed_front));
    println!("Part 1: {}", sum);

    // sum after 153rd generation equals 8575, every single generation afterwards adds 53
    let generation: u64 = 50_000_000_000;
    let sum = (generation - 153) * 53 + 8575;
    println!("Part 2: {}", sum);
}
