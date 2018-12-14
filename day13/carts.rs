use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::iter::FromIterator;

#[derive(PartialEq)]
enum TrackElement {
    Vertical,
    Horizontal,
    Backslash,
    Slash,
    Intersection,
    Empty,
}

enum IntersectionDecision {
    Left,
    Straight,
    Right,
}

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

struct Cart {
    x: usize,
    y: usize,
    orientation: Orientation,
    decision: IntersectionDecision,
}

impl Cart {
    fn new(x: usize, y: usize, orientation: Orientation) -> Cart {
        Cart {
            x,
            y,
            orientation,
            decision: IntersectionDecision::Left,
        }
    }

    fn turn_left(&mut self) {
        match self.orientation {
            Orientation::Up => self.orientation = Orientation::Left,
            Orientation::Down => self.orientation = Orientation::Right,
            Orientation::Left => self.orientation = Orientation::Down,
            Orientation::Right => self.orientation = Orientation::Up,
        }
    }

    fn turn_right(&mut self) {
        match self.orientation {
            Orientation::Up => self.orientation = Orientation::Right,
            Orientation::Down => self.orientation = Orientation::Left,
            Orientation::Left => self.orientation = Orientation::Up,
            Orientation::Right => self.orientation = Orientation::Down,
        }
    }

    fn turn_on_intersection(&mut self) {
        match self.decision {
            IntersectionDecision::Left => {
                self.decision = IntersectionDecision::Straight;
                self.turn_left();
            }
            IntersectionDecision::Straight => {
                self.decision = IntersectionDecision::Right;
            }
            IntersectionDecision::Right => {
                self.decision = IntersectionDecision::Left;
                self.turn_right();
            }
        }
    }

    fn move_one_tick(&mut self, track: &Vec<Vec<TrackElement>>) -> (usize, usize) {
        match self.orientation {
            Orientation::Up => self.y -= 1,
            Orientation::Down => self.y += 1,
            Orientation::Left => self.x -= 1,
            Orientation::Right => self.x += 1,
        }

        match track[self.y][self.x] {
            TrackElement::Backslash => match self.orientation {
                Orientation::Up | Orientation::Down => self.turn_left(),
                Orientation::Left | Orientation::Right => self.turn_right(),
            },
            TrackElement::Slash => match self.orientation {
                Orientation::Up | Orientation::Down => self.turn_right(),
                Orientation::Left | Orientation::Right => self.turn_left(),
            },
            TrackElement::Intersection => self.turn_on_intersection(),
            TrackElement::Empty => panic!("({}, {}): cart should not be here :|", self.x, self.y),
            _ => (),
        }

        (self.x, self.y)
    }
}

fn check_if_carts_collide(carts_positions: &VecDeque<(usize, usize)>) -> bool {
    let unique: HashSet<(usize, usize)> = HashSet::from_iter(carts_positions.iter().cloned());
    unique.len() != carts_positions.len()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut track = Vec::new();
    let mut carts = Vec::new();
    let mut positions = VecDeque::new();

    for (y, line) in content.lines().enumerate() {
        let mut row = Vec::new();

        for (x, el) in line.chars().enumerate() {
            match el {
                '|' => row.push(TrackElement::Vertical),
                '-' => row.push(TrackElement::Horizontal),
                '/' => row.push(TrackElement::Slash),
                '\\' => row.push(TrackElement::Backslash),
                '+' => row.push(TrackElement::Intersection),
                ' ' => row.push(TrackElement::Empty),
                '^' => {
                    row.push(TrackElement::Vertical);
                    carts.push(Cart::new(x, y, Orientation::Up));
                }
                'v' => {
                    row.push(TrackElement::Vertical);
                    carts.push(Cart::new(x, y, Orientation::Down));
                }
                '<' => {
                    row.push(TrackElement::Horizontal);
                    carts.push(Cart::new(x, y, Orientation::Left));
                }
                '>' => {
                    row.push(TrackElement::Horizontal);
                    carts.push(Cart::new(x, y, Orientation::Right));
                }
                _ => panic!("Unknown track element!"),
            }
        }
        track.push(row);
    }

    let mut crashed = Vec::new();
    let mut first_crash = true;

    loop {
        if carts.len() == 1 {
            println!("Part 2: ({}, {})", carts[0].x, carts[0].y);
            break;
        }
        let width = track.first().unwrap().len();
        carts.sort_by(|cart_a, cart_b| {
            (cart_a.y * width + cart_a.x).cmp(&(cart_b.y * width + cart_b.x))
        });
        positions.clear();
        carts
            .iter()
            .for_each(|cart| positions.push_back((cart.x, cart.y)));

        for cart in carts.iter_mut() {
            if crashed.contains(&(cart.x, cart.y)) {
                continue;
            }

            let new_position = cart.move_one_tick(&track);
            positions.pop_front();
            positions.push_back(new_position);

            if check_if_carts_collide(&positions) {
                if first_crash {
                    println!("Part 1: {:?}", new_position);
                    first_crash = false;
                }
                crashed.push(new_position);
                positions.pop_back();
            }
        }
        carts = carts
            .into_iter()
            .filter(|cart| !crashed.contains(&(cart.x, cart.y)))
            .collect();
        crashed.clear();
    }
}
