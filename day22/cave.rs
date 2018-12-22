const DEPTH: usize = 3339;
const TARGET_X: usize = 10;
const TARGET_Y: usize = 715;
const SIZE_X: usize = TARGET_X + 1;
const SIZE_Y: usize = TARGET_Y + 1;

#[derive(Clone, PartialEq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

fn calculate_erosion_levels(cave: &mut Vec<Vec<usize>>) {
    for x in 0..=TARGET_X {
        for y in 0..=TARGET_Y {
            if (x == 0 && y == 0) || (x == TARGET_X && y == TARGET_Y) {
                cave[y][x] = (0 + DEPTH) % 20183;
            } else if y == 0 {
                cave[y][x] = (x * 16807 + DEPTH) % 20183;
            } else if x == 0 {
                cave[y][x] = (y * 48271 + DEPTH) % 20183;
            } else {
                cave[y][x] = (cave[y][x - 1] * cave[y - 1][x] + DEPTH) % 20183;
            }
        }
    }
}

fn calculate_region_types(
    cave_erosion: &mut Vec<Vec<usize>>,
    cave_region_types: &mut Vec<Vec<RegionType>>,
) {
    for y in 0..=TARGET_Y {
        for x in 0..=TARGET_X {
            let region_type = cave_erosion[y][x] % 3;
            if region_type == 0 {
                cave_region_types[y][x] = RegionType::Rocky;
            } else if region_type == 1 {
                cave_region_types[y][x] = RegionType::Wet;
            } else if region_type == 2 {
                cave_region_types[y][x] = RegionType::Narrow;
            }
        }
    }
}

fn calulate_risk(cave_region_types: Vec<&RegionType>) -> usize {
    cave_region_types.iter().fold(0, |acc, region| {
        let mut region_risk = 0;
        if **region == RegionType::Wet {
            region_risk = 1;
        } else if **region == RegionType::Narrow {
            region_risk = 2;
        }
        acc + region_risk
    })
}

fn main() {
    let mut cave_erosion = vec![vec![0; SIZE_X]; SIZE_Y];
    calculate_erosion_levels(&mut cave_erosion);

    let mut cave_region_types = vec![vec![RegionType::Rocky; SIZE_X]; SIZE_Y];
    calculate_region_types(&mut cave_erosion, &mut cave_region_types);

    println!(
        "Risk: {}",
        calulate_risk(cave_region_types.iter().flatten().collect::<Vec<_>>())
    );
}
