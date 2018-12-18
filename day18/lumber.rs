use std::fs;

const SIZE: usize = 50;

fn determine_next(x: usize, y: usize, area: &[[char; SIZE]; SIZE]) -> char {
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

fn main() {
    let mut area = [['.'; SIZE]; SIZE];

    for (i, line) in fs::read_to_string("input").unwrap().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            area[i][j] = c;
        }
    }

    for _ in 0..10 {
        let mut new_area = [['.'; SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                new_area[i][j] = determine_next(j, i, &area);
            }
        }
        area = new_area;
    }

    let mut number_of_trees = 0;
    let mut number_of_lumberyards = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if area[i][j] == '|' {
                number_of_trees += 1;
            } else if area[i][j] == '#' {
                number_of_lumberyards += 1;
            }
        }
    }

    println!("{}", number_of_trees * number_of_lumberyards);
}
