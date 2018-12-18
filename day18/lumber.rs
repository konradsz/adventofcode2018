use std::fs;

const SIZE: usize = 50;

fn determine_next(x: usize, y: usize, area: &Vec<Vec<char>>) -> char {
    let mut adjacent_trees = 0;
    let mut adjacent_lumberyards = 0;

    let adjacents = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for adjacent in &adjacents {
        let coords = (x as i32 + adjacent.0, y as i32 + adjacent.1);
        if coords.0 >= 0 && coords.1 >= 0 && coords.0 < SIZE as i32 && coords.1 < SIZE as i32 {
            match area[coords.1 as usize][coords.0 as usize] {
                '|' => adjacent_trees += 1,
                '#' => adjacent_lumberyards += 1,
                _ => (),
            }
        }
    }
    match area[y][x] {
        '.' => {
            if adjacent_trees >= 3 {
                return '|';
            } else {
                return '.';
            }
        }
        '|' => {
            if adjacent_lumberyards >= 3 {
                return '#';
            } else {
                return '|';
            }
        }
        '#' => {
            if adjacent_lumberyards >= 1 && adjacent_trees >= 1 {
                return '#';
            } else {
                return '.';
            }
        }
        _ => (),
    }
    '.'
}

fn calculate_resource_value(areas: &Vec<Vec<char>>) -> usize {
    let number_of_trees = areas.iter().fold(0, |trees, area| {
        trees + area.iter().filter(|&&c| c == '|').count()
    });
    let number_of_lumberyards = areas.iter().fold(0, |lumberyards, area| {
        lumberyards + area.iter().filter(|&&c| c == '#').count()
    });

    number_of_trees * number_of_lumberyards
}

fn main() {
    let mut area = vec![vec!['.'; SIZE]; SIZE];

    for (i, line) in fs::read_to_string("input").unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            area[i][j] = c;
        }
    }

    let mut seen_areas = Vec::new();
    loop {
        if seen_areas.contains(&area) {
            break;
        }
        seen_areas.push(area.to_vec());
        let mut new_area = vec![vec!['.'; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                new_area[i][j] = determine_next(j, i, &area);
            }
        }
        area = new_area;
    }

    let previously_seen_index = seen_areas.iter().position(|a| *a == area).unwrap();
    let cycle_size = seen_areas.len() - previously_seen_index;
    let area_1000000000 = previously_seen_index + (1_000_000_000 - seen_areas.len()) % cycle_size;

    println!("Part 1: {}", calculate_resource_value(&seen_areas[10]));
    println!(
        "Part 2: {}",
        calculate_resource_value(&seen_areas[area_1000000000])
    );
}
