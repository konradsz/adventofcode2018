use std::fs;

struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: u32,
}

impl Nanobot {
    fn calculate_manhattan_distance(&self, nanobot: &Nanobot) -> u32 {
        (self.x - nanobot.x).abs() as u32
            + (self.y - nanobot.y).abs() as u32
            + (self.z - nanobot.z).abs() as u32
    }

    fn calculate_manhattan_distance_from_coords(&self, x: i32, y: i32, z: i32) -> u32 {
        (self.x - x).abs() as u32 + (self.y - y).abs() as u32 + (self.z - z).abs() as u32
    }
}

fn calculate_manhattan_distance_from_origin(x: i32, y: i32, z: i32) -> u32 {
    (x.abs() + y.abs() + z.abs()) as u32
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut nanobots = Vec::new();

    for line in content.lines() {
        let (_, payload) = line.split_at(5);
        let mut parts = payload.split('>');
        let coords = parts.next().unwrap();
        let (_, radius) = parts.next().unwrap().split_at(4);

        let coords: Vec<i32> = coords
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();
        let radius: u32 = radius.parse().unwrap();

        nanobots.push(Nanobot {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            r: radius,
        });
    }

    let max_radius_nanobot = nanobots
        .iter()
        .max_by(|nanobot_1, nanobot_2| nanobot_1.r.cmp(&nanobot_2.r))
        .unwrap();
    let nanobots_within_radius = nanobots
        .iter()
        .filter(|nanobot| {
            nanobot.calculate_manhattan_distance(&max_radius_nanobot) <= max_radius_nanobot.r
        }).count();

    println!("Part 1: {}", nanobots_within_radius);

    let mut min_x = nanobots
        .iter()
        .min_by(|nanobot_1, nanobot_2| nanobot_1.x.cmp(&nanobot_2.x))
        .unwrap()
        .x;
    let mut max_x = nanobots
        .iter()
        .max_by(|nanobot_1, nanobot_2| nanobot_1.x.cmp(&nanobot_2.x))
        .unwrap()
        .x;
    let mut min_y = nanobots
        .iter()
        .min_by(|nanobot_1, nanobot_2| nanobot_1.y.cmp(&nanobot_2.y))
        .unwrap()
        .y;
    let mut max_y = nanobots
        .iter()
        .max_by(|nanobot_1, nanobot_2| nanobot_1.y.cmp(&nanobot_2.y))
        .unwrap()
        .y;
    let mut min_z = nanobots
        .iter()
        .min_by(|nanobot_1, nanobot_2| nanobot_1.z.cmp(&nanobot_2.z))
        .unwrap()
        .z;
    let mut max_z = nanobots
        .iter()
        .max_by(|nanobot_1, nanobot_2| nanobot_1.z.cmp(&nanobot_2.z))
        .unwrap()
        .z;

    let mut range = 1;
    while range < max_x - min_x {
        range *= 2;
    }

    loop {
        let mut nanobots_in_range_best = 0;
        let (mut x_best, mut y_best, mut z_best) = (0, 0, 0);
        let mut distance_from_origin_best = std::u32::MAX;

        for x in (min_x..max_x).step_by(range as usize) {
            for y in (min_y..max_y).step_by(range as usize) {
                for z in (min_z..max_z).step_by(range as usize) {
                    let nanobots_in_range = nanobots
                        .iter()
                        .filter(|nanobot| {
                            (nanobot.calculate_manhattan_distance_from_coords(x, y, z) as i64
                                - nanobot.r as i64)
                                / range as i64
                                <= 0
                        }).count();

                    if nanobots_in_range > nanobots_in_range_best {
                        nanobots_in_range_best = nanobots_in_range;
                        x_best = x;
                        y_best = y;
                        z_best = z;
                        distance_from_origin_best =
                            calculate_manhattan_distance_from_origin(x, y, z);
                    } else if nanobots_in_range == nanobots_in_range_best {
                        let current_distance = calculate_manhattan_distance_from_origin(x, y, z);
                        if current_distance < distance_from_origin_best {
                            nanobots_in_range_best = nanobots_in_range;
                            x_best = x;
                            y_best = y;
                            z_best = z;
                            distance_from_origin_best = current_distance;
                        }
                    }
                }
            }
        }

        if range == 1 {
            println!("Part 2: {}", distance_from_origin_best);
            break;
        }

        min_x = x_best - range;
        max_x = x_best + range;
        min_y = y_best - range;
        max_y = y_best + range;
        min_z = z_best - range;
        max_z = z_best + range;

        range /= 2;
    }
}
