use std::fs;

#[derive(Clone)]
struct Point(i32, i32, i32, i32);
struct Constellation(Vec<Point>);

fn manhattan_distance(point_a: &Point, point_b: &Point) -> i32 {
    (point_a.0 - point_b.0).abs()
        + (point_a.1 - point_b.1).abs()
        + (point_a.2 - point_b.2).abs()
        + (point_a.3 - point_b.3).abs()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut constellations: Vec<Constellation> = Vec::new();

    for line in content.lines() {
        let new_point: Vec<i32> = line
            .split(',')
            .map(|number| number.parse().unwrap())
            .collect();
        let new_point = Point(new_point[0], new_point[1], new_point[2], new_point[3]);

        let mut matching_points: Vec<Point> = constellations
            .iter()
            .filter(|constellation| {
                constellation
                    .0
                    .iter()
                    .any(|point| manhattan_distance(&point, &new_point) <= 3)
            }).flat_map(|c| c.0.clone())
            .collect();

        let not_matching_constellations = constellations
            .into_iter()
            .filter(|constellation| {
                constellation
                    .0
                    .iter()
                    .all(|point| manhattan_distance(&point, &new_point) > 3)
            }).collect::<Vec<_>>();

        matching_points.push(new_point);
        constellations = not_matching_constellations;
        constellations.push(Constellation(matching_points));
    }

    println!("{}", constellations.len());
}
