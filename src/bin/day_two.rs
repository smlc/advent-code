use std::collections::HashMap;
fn main() {

    let total_score_one = part_one_solution();
    println!("Part one total score is {total_score_one}");

    let total_score_two = part_two_solution();
    println!("Part two total score is {total_score_two}");
}
fn part_one_solution() -> i32 {
    let mut total_score : i32 = 0;
    // A Rock(1), B Paper(2), C Scissors(3)
    // X Rock(1), Y Paper(2), Z Scissors(3), My play
    let scores = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3)
    ]);
    let rounds: Vec<&str> = include_str!("input.txt")
            .split("\n").collect();

    
    for round in rounds {
        if let Some(score) = scores.get(round) {
            total_score = score + total_score;
        }
    }

    total_score
}

fn part_two_solution() -> i32 {
    let mut total_score : i32 = 0;
    // A Rock(1), B Paper(2), C Scissors(3)
    // X Lose(0), Y Draw(3), Z Win(6), My play
    let scores = HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6)
    ]);
    let rounds: Vec<&str> = include_str!("input.txt")
            .split("\n").collect();

    
    for round in rounds {
        if let Some(score) = scores.get(round) {
            total_score = score + total_score;
        }
    }

    total_score
}