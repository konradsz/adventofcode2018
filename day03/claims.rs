use std::cmp;
use std::collections::HashSet;
use std::fs;

struct Rectangle {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn new(claim: &str) -> Rectangle {
        let mut parts = claim.split_whitespace();
        let id = parts.next().unwrap().trim_matches('#').parse().unwrap();
        parts.next().unwrap(); // @
        let margins: Vec<usize> = parts
            .next()
            .unwrap()
            .trim_matches(':')
            .split(',')
            .map(|margin| margin.parse().unwrap())
            .collect();
        let size: Vec<usize> = parts
            .next()
            .unwrap()
            .split('x')
            .map(|size| size.parse().unwrap())
            .collect();

        Rectangle {
            id,
            x: margins[0],
            y: margins[1],
            width: size[0],
            height: size[1],
        }
    }
}

fn calculate_grid_size(rectangles: &Vec<Rectangle>) -> usize {
    let grid_width = rectangles
        .iter()
        .max_by(|r1, r2| (r1.x + r1.width).cmp(&(r2.x + r2.width)))
        .map(|r| r.x + r.width)
        .unwrap();
    let grid_height = rectangles
        .iter()
        .max_by(|r1, r2| (r1.y + r1.height).cmp(&(r2.y + r2.height)))
        .map(|r| r.y + r.height)
        .unwrap();

    cmp::max(grid_width, grid_height)
}

fn main() {
    let mut rectangles = Vec::new();

    for line in fs::read_to_string("input").unwrap().lines() {
        rectangles.push(Rectangle::new(line));
    }

    let grid_size = calculate_grid_size(&rectangles);
    let mut grid: Vec<Vec<i32>> = vec![vec![0; grid_size]; grid_size];

    let mut overlapped = 0;
    let mut intact_claims = HashSet::new();
    for rectangle in &rectangles {
        intact_claims.insert(rectangle.id);
        for i in rectangle.x..rectangle.x + rectangle.width {
            for j in rectangle.y..rectangle.y + rectangle.height {
                //if grid[i]
                if grid[i][j] == 0 {
                    grid[i][j] = rectangle.id as i32;
                } else {
                    intact_claims.remove(&rectangle.id);
                    if grid[i][j] != -1 {
                        intact_claims.remove(&(grid[i][j] as u32));
                        grid[i][j] = -1;
                        overlapped += 1;
                    }
                }
            }
        }
    }
    println!("{}", overlapped);
    println!("{}", intact_claims.iter().next().unwrap());
}
