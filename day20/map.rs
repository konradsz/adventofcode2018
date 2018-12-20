use std::collections::HashMap;
use std::cmp;
use std::fs;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let input = content.trim().trim_start_matches('^').trim_end_matches('$');
    let mut current_position = Coordinate {x: 0, y: 0};
    let mut previous_position = Coordinate {x: 0, y: 0};
    let mut coordinates = vec![current_position.clone()];

    let mut distances = HashMap::new();
    distances.insert(current_position.clone(), 0);

    let mut movement = HashMap::new();
    movement.insert('N', (0, -1));
    movement.insert('E', (1, 0));
    movement.insert('S', (0, 1));
    movement.insert('W', (-1, 0));

    let mut intersections = Vec::new();
    for c in input.chars() {
        match c {
            '(' => intersections.push(current_position.clone()),
            ')' => current_position = intersections.pop().unwrap(),
            '|' => current_position = intersections.last().unwrap().clone(),
            'N' | 'E' | 'S' | 'W' => {
                current_position.x += movement.get(&c).unwrap().0;
                current_position.y += movement.get(&c).unwrap().1;
                
                coordinates.push(current_position.clone());

                let previous_distance = distances[&previous_position];
                if distances.contains_key(&current_position) {
                    let distance = distances[&current_position];
                    if let Some(value) = distances.get_mut(&current_position) {
                        *value = cmp::min(distance, previous_distance + 1);
                    }

                } else {
                    distances.insert(current_position.clone(), previous_distance + 1);
                }
            },
            _ => panic!("Unknown element: {}", c)
        }

        previous_position = current_position.clone();
    }

    let max_number_of_doors = distances.values().max().unwrap();
    println!("Part 1: {}", max_number_of_doors);

    let rooms_behind_at_least_1000_doors = distances.values().filter(|&&v| v >= 1000).count();
    println!("Part 2: {}", rooms_behind_at_least_1000_doors);
}
