use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum GameResult {
    WIN = 6,
    DRAW = 3,
    LOSS = 0,
}

enum Choice {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

/// A, X -> Rock
/// B, Y -> Paper
/// C, Z -> Scissors
fn get_score(their: &str, ours: &str) -> u32 {
    let score = match their {
        "A" => match ours {
            // Rock
            "X" => GameResult::DRAW as u32 + Choice::ROCK as u32,
            "Y" => GameResult::WIN as u32 + Choice::PAPER as u32,
            "Z" => GameResult::LOSS as u32 + Choice::SCISSORS as u32,
            _ => panic!("Bad option"),
        },
        "B" => match ours {
            // Paper
            "X" => GameResult::LOSS as u32 + Choice::ROCK as u32,
            "Y" => GameResult::DRAW as u32 + Choice::PAPER as u32,
            "Z" => GameResult::WIN as u32 + Choice::SCISSORS as u32,
            _ => panic!("Bad option"),
        },
        "C" => match ours {
            // Scissors
            "X" => GameResult::WIN as u32 + Choice::ROCK as u32,
            "Y" => GameResult::LOSS as u32 + Choice::PAPER as u32,
            "Z" => GameResult::DRAW as u32 + Choice::SCISSORS as u32,
            _ => panic!("Bad option"),
        },
        _ => panic!("Bad option"),
    };

    score
}

/// A, X -> Need to lose
/// B, Y -> Need to draw
/// C, Z -> Need to win
fn get_score_second_part(their: &str, ours: &str) -> u32 {
    let score = match their {
        "A" => match ours {
            // Rock
            "X" => GameResult::LOSS as u32 + Choice::SCISSORS as u32,
            "Y" => GameResult::DRAW as u32 + Choice::ROCK as u32,
            "Z" => GameResult::WIN as u32 + Choice::PAPER as u32,
            _ => panic!("Bad option"),
        },
        "B" => match ours {
            // Paper
            "X" => GameResult::LOSS as u32 + Choice::ROCK as u32,
            "Y" => GameResult::DRAW as u32 + Choice::PAPER as u32,
            "Z" => GameResult::WIN as u32 + Choice::SCISSORS as u32,
            _ => panic!("Bad option"),
        },
        "C" => match ours {
            // Scissors
            "X" => GameResult::LOSS as u32 + Choice::PAPER as u32,
            "Y" => GameResult::DRAW as u32 + Choice::SCISSORS as u32,
            "Z" => GameResult::WIN as u32 + Choice::ROCK as u32,
            _ => panic!("Bad option"),
        },
        _ => panic!("Bad option"),
    };

    score
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total: u32 = 0;
    let mut total_second_part: u32 = 0;

    for line in reader.lines() {
        let line_un = line.unwrap();
        let round: Vec<&str> = line_un.split(" ").collect();

        let score = get_score(round[0], round[1]);
        total += score;

        let score_second_part = get_score_second_part(round[0], round[1]);
        total_second_part += score_second_part;
    }

    println!("{}", total);
    println!("{}", total_second_part);
}
