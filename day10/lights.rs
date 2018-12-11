use std::fs;

#[derive(Clone)]
struct Light {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

fn find_min_max(lights: &Vec<Light>) -> ((i32, i32), (i32, i32)) {
    let min_x = lights
        .iter()
        .min_by(|light_a, light_b| light_a.x.cmp(&light_b.x))
        .unwrap()
        .x;
    let max_x = lights
        .iter()
        .max_by(|light_a, light_b| light_a.x.cmp(&light_b.x))
        .unwrap()
        .x;
    let min_y = lights
        .iter()
        .min_by(|light_a, light_b| light_a.y.cmp(&light_b.y))
        .unwrap()
        .y;
    let max_y = lights
        .iter()
        .max_by(|light_a, light_b| light_a.y.cmp(&light_b.y))
        .unwrap()
        .y;

    ((min_x, max_x), (min_y, max_y))
}

fn calculate_grid_size(min_max_x: (i32, i32), min_max_y: (i32, i32)) -> (usize, usize) {
    (
        (min_max_x.1 - min_max_x.0 + 1) as usize,
        (min_max_y.1 - min_max_y.0 + 1) as usize,
    )
}

fn update(lights: &mut Vec<Light>) {
    for light in lights {
        light.x += light.velocity_x;
        light.y += light.velocity_y;
    }
}

fn print(lights: &Vec<Light>, width: usize, height: usize, min_x: i32, min_y: i32) {
    let mut grid = vec![vec![' '; width]; height];

    for light in lights {
        grid[(light.y - min_y) as usize][(light.x - min_x) as usize] = '*';
    }

    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() {
    let mut lights = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let mut part_iter = line.split('>');
        let position = part_iter.next().unwrap().trim_start_matches("position=<");
        let velocity = part_iter.next().unwrap().trim_start_matches(" velocity=<");
        let position: Vec<i32> = position
            .split(',')
            .map(|c| c.trim().parse::<i32>().unwrap())
            .collect();
        let velocity: Vec<i32> = velocity
            .split(',')
            .map(|c| c.trim().parse::<i32>().unwrap())
            .collect();

        lights.push(Light {
            x: position[0],
            y: position[1],
            velocity_x: velocity[0],
            velocity_y: velocity[1],
        });
    }

    let mut previous_area = std::usize::MAX;
    let mut previous_lights = lights.to_vec();
    let mut seconds = 0;
    loop {
        let ((min_x, max_x), (min_y, max_y)) = find_min_max(&lights);
        let (width, height) = calculate_grid_size((min_x, max_x), (min_y, max_y));
        let area = width * height;
        if area < previous_area {
            previous_area = area;
            previous_lights = lights.to_vec();
        } else {
            let ((min_x, max_x), (min_y, max_y)) = find_min_max(&previous_lights);
            let (width, height) = calculate_grid_size((min_x, max_x), (min_y, max_y));
            print(&previous_lights, width, height, min_x, min_y);
            println!("{}", seconds - 1);
            break;
        }
        update(&mut lights);
        seconds += 1;
    }
}
