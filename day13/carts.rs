use std::fs;
use std::collections::VecDeque;
use std::fmt;
use std::collections::HashSet;
use std::iter::FromIterator;

// add printing mechanism for Element
#[derive(PartialEq)]
enum TrackElement {
    Vertical,
    Horizontal,
    Backslash,
    Slash,
    Intersection,
    Cart,
    Empty
}

enum IntersectionOption {
    Left,
    Straight,
    Right
}

enum CartOrientation {
    Up,
    Down,
    Left,
    Right
}

struct Cart {
    x: usize,
    y: usize,
    orientation: CartOrientation,
    next_option: IntersectionOption,
    has_crashed: bool
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Cart {
    fn new(x: usize, y: usize, orientation: CartOrientation) -> Cart {
        Cart {
            x,
            y,
            orientation,
            next_option: IntersectionOption::Left,
            has_crashed: false
        }
    }

    fn turn_left(&mut self) {
        match self.orientation {
            CartOrientation::Up => self.orientation = CartOrientation::Left,
            CartOrientation::Down => self.orientation = CartOrientation::Right,
            CartOrientation::Left => self.orientation = CartOrientation::Down,
            CartOrientation::Right => self.orientation = CartOrientation::Up,
        }
    }

    fn turn_right(&mut self) {
        match self.orientation {
            CartOrientation::Up => self.orientation = CartOrientation::Right,
            CartOrientation::Down => self.orientation = CartOrientation::Left,
            CartOrientation::Left => self.orientation = CartOrientation::Up,
            CartOrientation::Right => self.orientation = CartOrientation::Down,
        }
    }

    fn turn_on_intersection(&mut self) {
        match self.next_option {
            IntersectionOption::Left => {
                self.next_option = IntersectionOption::Straight;
                self.turn_left();
            },
            IntersectionOption::Straight => {
                self.next_option = IntersectionOption::Right;
            },
            IntersectionOption::Right => {
                self.next_option = IntersectionOption::Left;
                self.turn_right();
            }
        }
    }

    fn move_one_tick(&mut self, track: &Vec<Vec<TrackElement>>) -> (usize, usize) {
        match self.orientation {
            CartOrientation::Up => self.y -= 1,
            CartOrientation::Down => self.y += 1,
            CartOrientation::Left => self.x -= 1,
            CartOrientation::Right => self.x += 1
        }

        match track[self.y][self.x] {
            TrackElement::Backslash => {
                match self.orientation {
                    CartOrientation::Up | CartOrientation::Down => self.turn_left(),
                    CartOrientation::Left | CartOrientation::Right => self.turn_right(),
                }
            },
            TrackElement::Slash => {
                match self.orientation {
                    CartOrientation::Up | CartOrientation::Down=> self.turn_right(),
                    CartOrientation::Left | CartOrientation::Right => self.turn_left(),
                }
            },
            TrackElement::Intersection => self.turn_on_intersection(),
            TrackElement::Empty => panic!("({}, {}): cart should not be here :|", self.x, self.y),
            _ => ()
        }

        (self.x, self.y)
    }
}

fn print_track(track: &Vec<Vec<TrackElement>>, carts: &Vec<Cart>) {
    for (y, row) in track.iter().enumerate() {
        'outer: for (x, c) in row.iter().enumerate() {
            let mut carts_iter = carts.iter().filter(|cart| cart.x == x && cart.y == y);
            if carts.iter().filter(|cart| cart.x == x && cart.y == y).count() > 1 {
                print!("X");
                continue 'outer;
            } else {
                for cart in carts_iter {
                    match cart.orientation {
                        CartOrientation::Up => print!("^"),
                        CartOrientation::Down => print!("v"),
                        CartOrientation::Left => print!("<"),
                        CartOrientation::Right => print!(">")
                    }
                    continue 'outer;
                }
            }

            if *c == TrackElement::Vertical {
                print!("|");
            } else if *c == TrackElement::Horizontal {
                print!("-");
            } else if *c == TrackElement::Slash {
                print!("/");
            } else if *c == TrackElement::Backslash {
                print!("\\");
            } else if *c == TrackElement::Intersection {
                print!("+");
            } else if *c == TrackElement::Cart {
                print!("O");
            } else if *c == TrackElement::Empty {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}

fn check_if_carts_collide(carts_positions: &VecDeque<(usize, usize)>) -> bool {
    //carts.iter().zip(carts.iter().skip(1)).any(|(cart_a, cart_b)| cart_a.x == cart_b.x && cart_a.y == cart_b.y)

    carts_positions.iter().zip(carts_positions.iter().skip(1)).any(|(cart_a, cart_b)| cart_a.0 == cart_b.0 && cart_a.1 == cart_b.1);
    //let mut unique = HashSet::new();
    //carts_positions.iter().for_each(|cart| unique.insert((cart.0, cart.1)));
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
            // MATCH !
            if el == '|' {
                row.push(TrackElement::Vertical);
            } else if el == '-' {
                row.push(TrackElement::Horizontal);
            } else if el == '/' {
                row.push(TrackElement::Slash);
            } else if el == '\\' {
                row.push(TrackElement::Backslash);
            } else if el == '+' {
                row.push(TrackElement::Intersection);
            } else if el == '^' { // merge ^ with v (?)
                //row.push(TrackElement::Cart);
                row.push(TrackElement::Vertical);
                carts.push(Cart::new(x, y, CartOrientation::Up));
                positions.push_back((x, y));
            } else if el == 'v' { // merge ^ with v (?)
                //row.push(TrackElement::Cart);
                row.push(TrackElement::Vertical);
                carts.push(Cart::new(x, y, CartOrientation::Down));
                positions.push_back((x, y));
            } else if el == '<' { // merge < with > (?)
                //row.push(TrackElement::Cart);
                row.push(TrackElement::Horizontal);
                carts.push(Cart::new(x, y, CartOrientation::Left));
                positions.push_back((x, y));
            } else if el == '>' { // merge < with > (?)
                //row.push(TrackElement::Cart);
                row.push(TrackElement::Horizontal);
                carts.push(Cart::new(x, y, CartOrientation::Right));
                positions.push_back((x, y));
            } else if el == ' ' {
                row.push(TrackElement::Empty);
            }
        }
        track.push(row);
    }
//|| el == 'v' || el == '<' || el == '>' {
    //print_track(&track, &carts);


    let mut crashed = Vec::new();
    'ticks: loop {
        println!("TICK");
        println!("Track before tick");
        //print_track(&track, &carts);
        if carts.len() == 1 {
            println!("{}, {}", carts[0].x, carts[0].y);
            break;
        }
        let width = track.first().unwrap().len();
        carts.sort_by(|cart_a, cart_b| (cart_a.y * width + cart_a.x).cmp(&(cart_b.y * width + cart_b.x)));
        positions.clear();
        carts.iter().for_each(|cart| positions.push_back((cart.x, cart.y)));
        //println!("{}", positions.len());
        //println!("{:?}", positions);
        for cart in carts.iter_mut() {
            println!("{:?}", positions);
            if crashed.contains(&(cart.x, cart.y)) {
                //positions.pop_front();
                continue;
            }
            println!("before move: {:?}", (cart.x, cart.y));
            let new_position = cart.move_one_tick(&track);
            println!("after move: {:?}", (cart.x, cart.y));
            //println!("{:?}", new_position);
            positions.pop_front();
            positions.push_back(new_position);
//            println!("{:?}", positions);
println!("{:?}", positions);

            if check_if_carts_collide(&positions) {
                println!("BOOM at {:?}", new_position);
                crashed.push(new_position);
                positions.pop_back();
                //break 'ticks;
            }
        }
        carts = carts.into_iter().filter(|cart| !crashed.contains(&(cart.x, cart.y))).collect();
        crashed.clear();
        //println!("carts: {}, crashed: {:?}", carts.len(), crashed);
        /*carts = carts
                .into_iter()
                .filter(|cart| !crashed.contains(&c.0))
                .collect();*/

        //carts.sort_by(|cart_a, cart_b| cart_a.y.cmp(&cart_b.y));




        /*for cart in carts.iter() {
            print!("{}", cart);
        }println!("");*/
        //carts.iter().zip(carts.iter()).filter(|(cart_a, cart_b)| cart_a.y != cart_b.y).for_each(|cart| print!("ASD")/*println!("({}, {})", (&cart).x, *cart.y)*/);
//println!("{:?}", positions);
//positions.iter().zip(positions.iter().skip(1)).filter(|(pos_a, pos_b)| pos_a.1 == pos_b.1).for_each(|pos| print!("{:?}", pos));println!("");
//println!("Track after tick");
//print_track(&track, &carts);
    }

    //print_track(&track, &carts);
}
