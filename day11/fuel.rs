fn calculate_total_power(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id: i32 = x + 10;
    let mut total_power: i32 = 0;
    total_power += rack_id;
    total_power *= y;
    total_power += serial_number;
    total_power *= rack_id;
    total_power = total_power
        .to_string()
        .chars()
        .nth(total_power.to_string().len() - 3)
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;
    total_power -= 5;
    total_power
}

fn calculate_cell_power(grid: &Vec<Vec<i32>>, x: usize, y: usize, cell_size: usize) -> i32 {
    let mut cell_power = 0;
    for i in y - 1..y + cell_size - 1 {
        for j in x - 1..x + cell_size - 1 {
            cell_power += grid[i][j];
        }
    }

    cell_power
}

fn main() {
    const SIZE: usize = 300;
    const GRID_SERIAL_NUMBER: i32 = 8141;
    let mut grid = vec![vec![0; SIZE]; SIZE];
    for y in 1..=SIZE {
        for x in 1..=SIZE {
            grid[y - 1][x - 1] = calculate_total_power(x as i32, y as i32, GRID_SERIAL_NUMBER);
        }
    }

    // Part 1
    const CELL_SIZE: usize = 3;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_power = 0;

    for y in 1..=SIZE - CELL_SIZE {
        for x in 1..=SIZE - CELL_SIZE {
            let power = calculate_cell_power(&grid, x, y, CELL_SIZE);
            if power > max_power {
                max_power = power;
                max_x = x;
                max_y = y;
            }
        }
    }
    println!("{},{}", max_x, max_y);

    // Part 2
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_power = 0;
    let mut best_cell_size = 0;

    for cell_size in 0..SIZE {
        for y in 1..=SIZE - cell_size {
            for x in 1..=SIZE - cell_size {
                let power = calculate_cell_power(&grid, x, y, cell_size + 1);
                if power > max_power {
                    max_power = power;
                    max_x = x;
                    max_y = y;
                    best_cell_size = cell_size + 1;
                }
            }
        }
    }
    println!("{},{},{}", max_x, max_y, best_cell_size);
}
