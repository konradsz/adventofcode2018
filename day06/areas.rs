use std::fs;

struct Coordinate {
    x: usize,
    y: usize,
}

fn calculate_manhattan_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
}

fn main() {
    let mut coordinates = Vec::new();
    for line in fs::read_to_string("input").unwrap().lines() {
        let mut entry = line.split(',');
        coordinates.push(Coordinate {
            x: entry.next().unwrap().trim().parse().unwrap(),
            y: entry.next().unwrap().trim().parse().unwrap(),
        });
    }

    let grid_width = coordinates.iter().map(|coord| coord.x).max().unwrap() + 1;
    let grid_height = coordinates.iter().map(|coord| coord.y).max().unwrap() + 1;

    let mut grid = vec![vec![0; grid_width]; grid_height];
    let mut region_size = 0;

    for x in 0..grid_width {
        for y in 0..grid_height {
            let mut min_distance = std::usize::MAX;
            let mut total_distance = 0;

            for (index, coordinate) in coordinates.iter().enumerate() {
                let distance = calculate_manhattan_distance((x, y), (coordinate.x, coordinate.y));
                total_distance += distance;

                if distance == min_distance {
                    grid[y][x] = 0;
                } else if distance < min_distance {
                    min_distance = distance;
                    grid[y][x] = index + 1;
                }
            }

            if total_distance < 10_000 {
                region_size += 1;
            }
        }
    }

    let mut max_area = 0;
    for i in 1..coordinates.len() + 1 {
        let mut area = 0;
        'outer: for x in 0..grid_width {
            for y in 0..grid_height {
                if grid[y][x] == i {
                    area += 1;
                    if x == 0 || y == 0 || x == grid_width - 1 || y == grid_height - 1 {
                        area = 0;
                        break 'outer;
                    }
                }
            }
        }

        if area > max_area {
            max_area = area;
        }
    }

    println!("Part 1: {}", max_area);
    println!("Part 2: {}", region_size);
}
