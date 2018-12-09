use std::collections::VecDeque;

fn calculate_score(number_of_players: usize, max_marble_value: usize) -> usize {
    let mut marbles = VecDeque::new();
    marbles.push_front(0);
    let mut players_score = vec![0; number_of_players];

    for marble in 1..=max_marble_value {
        let current_player = marble % number_of_players;

        if marble % 23 == 0 {
            for _ in 0..7 {
                let m = marbles.pop_back().unwrap();
                marbles.push_front(m);
            }
            let score = marbles.pop_front().unwrap();
            players_score[current_player] += marble;
            players_score[current_player] += score;
        } else {
            for _ in 0..2 {
                let m = marbles.pop_front().unwrap();
                marbles.push_back(m);
            }
            marbles.push_front(marble);
        }
    }

    *players_score.iter().max().unwrap()
}

fn main() {
    println!("Part 1: {}", calculate_score(435, 71184));
    println!("Part 2: {}", calculate_score(435, 7118400));
}
