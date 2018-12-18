use std::fs;

#[derive(Default)]
struct ClayEntry {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize
}

fn flow(ground: &mut Vec<Vec<char>>, x: usize, y: usize) {
    if y == ground.len() || x == ground.first().unwrap().len() {
        return ();
    }

    if ground[y][x] == '.' {
        ground[y][x] = '|';
        flow(ground, x, y + 1);
    } else if ground[y][x] == '#' || ground[y][x] == '~' {
        let left = propagate_left(ground, x, y - 1);
        let right = propagate_right(ground, x, y - 1);

        if left && right {
            fill_level(ground, x, y - 1);
            flow(ground, x, y - 2);
        }
    } else if ground[y][x] == '|' {
        if (ground[y][x - 1] == '|' && ground[y][x + 1] == '|') || (ground[y + 1][x] == '|' && ground[y - 1][x] == '|') {
            return ();
        }

        let left = propagate_left(ground, x, y);
        let right = propagate_right(ground, x, y);

        if left && right {
            fill_level(ground, x, y);
            flow(ground, x, y - 1);
        }
    }
}

fn propagate_left(ground: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if ground[y + 1][x] != '#' && ground[y + 1][x] != '~' {
        flow(ground, x, y);
        return false;
    }

    if ground[y][x] == '.' || ground[y][x] == '|' {
        ground[y][x] = '|';

        return propagate_left(ground, x - 1, y);
    }
    true
}

fn propagate_right(ground: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if ground[y + 1][x] != '#' && ground[y + 1][x] != '~' {
        flow(ground, x, y);
        return false;
    }

    if ground[y][x] == '.' || ground[y][x] == '|' {
        ground[y][x] = '|';

        return propagate_right(ground, x + 1, y);
    }
    true
}

fn fill_level(ground: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let mut current_x = x + 1;
    while ground[y][current_x] == '|' {
        ground[y][current_x] = '~';
        current_x += 1;
    }

    current_x = x - 1;
    while ground[y][current_x] == '|' {
        ground[y][current_x] = '~';
        current_x -= 1;
    }

    ground[y][x] = '~';
}

fn main() {
    let mut clay_veins = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let mut coordinates = line.split(',');
        let matches: &[_] = &[' ', 'x', 'y', '='];

        let coord_1 = coordinates.next().unwrap();
        let coord_2 = coordinates.next().unwrap();
        let mut x;
        let mut y;
        if coord_1.contains("x") {
            x = coord_1.trim_matches(matches);
            y = coord_2.trim_matches(matches);
        } else {
            y = coord_1.trim_matches(matches);
            x = coord_2.trim_matches(matches);
        }

        let mut clay: ClayEntry = Default::default();
        if x.contains("..") {
            let mut x = x.split("..");
            clay.min_x = x.next().unwrap().parse().unwrap();
            clay.max_x = x.next().unwrap().parse().unwrap();
        }
        else {
            let x: usize = x.parse().unwrap();
            clay.min_x = x;
            clay.max_x = x;
        }

        if y.contains("..") {
            let mut y = y.split("..");
            clay.min_y = y.next().unwrap().parse().unwrap();
            clay.max_y = y.next().unwrap().parse().unwrap();
        }
        else {
            let y: usize = y.parse().unwrap();
            clay.min_y = y;
            clay.max_y = y;
        }
        clay_veins.push(clay);
    }

    let min_x = clay_veins.iter().min_by(|e_1, e_2| e_1.min_x.cmp(&e_2.min_x)).unwrap().min_x;
    let max_x = clay_veins.iter().max_by(|e_1, e_2| e_1.max_x.cmp(&e_2.max_x)).unwrap().max_x;
    let min_y = clay_veins.iter().min_by(|e_1, e_2| e_1.min_y.cmp(&e_2.min_y)).unwrap().min_y;
    let max_y = clay_veins.iter().max_by(|e_1, e_2| e_1.max_y.cmp(&e_2.max_y)).unwrap().max_y;

    let width = max_x - min_x;
    let height = max_y - min_y;

    let mut ground = Vec::new();
    for _ in 0..=height {
        let mut row = Vec::new();
        for _ in 0..=width + 2 {
            row.push('.');
        }
        ground.push(row);
    }

    for clay in &clay_veins {
        if clay.min_x != clay.max_x {
            for i in clay.min_x..=clay.max_x {
                ground[clay.min_y - min_y][i - min_x + 1] = '#';
            }
        } else if clay.min_y != clay.max_y {
            for i in clay.min_y..=clay.max_y {
                ground[i - min_y][clay.min_x - min_x + 1] = '#';
            }
        }
    }

    flow(&mut ground, 501 - min_x, 0);

    let mut total_flowing = 0;
    let mut total_retained = 0;
    for row in &ground {
        for c in row.iter() {
            if *c == '|' {
                total_flowing += 1;
            } else if *c == '~' {
                total_retained += 1;
            }
        }
    }

    println!("Part 1: {}", total_flowing + total_retained);
    println!("Part 2: {}", total_retained);
}
