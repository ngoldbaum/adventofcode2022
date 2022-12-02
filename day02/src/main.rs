use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let rounds = contents
        .trim()
        .split("\n")
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| (x[0].chars().next().unwrap(), x[1].chars().next().unwrap()))
        .collect::<Vec<(char, char)>>();

    let mut total_score: usize = 0;

    for round in rounds {
        let result = get_winner(round);
        total_score += result.value() + get_score(round.1);
    }

    dbg!(total_score);

    Ok(())
}

fn get_score(c: char) -> usize {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => panic!(),
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn value(&self) -> usize {
        match *self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }
}

fn get_winner(round: (char, char)) -> GameResult {
    match round {
        ('A', 'X') => GameResult::Draw,
        ('A', 'Y') => GameResult::Win,
        ('A', 'Z') => GameResult::Loss,
        ('B', 'X') => GameResult::Loss,
        ('B', 'Y') => GameResult::Draw,
        ('B', 'Z') => GameResult::Win,
        ('C', 'X') => GameResult::Win,
        ('C', 'Y') => GameResult::Loss,
        ('C', 'Z') => GameResult::Draw,
        _ => panic!(),
    }
}
