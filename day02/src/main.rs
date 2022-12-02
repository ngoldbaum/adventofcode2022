use std::error;
use std::fs::read_to_string;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let rounds = contents
        .trim()
        .split("\n")
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| (x[0].parse::<Move>().unwrap(), x[1].parse::<Move>().unwrap()))
        .collect::<Vec<(Move, Move)>>();

    let mut total_score: usize = 0;

    for round in rounds {
        let result = get_winner((&round.0, &round.1));
        total_score += result.value() + round.1.score();
    }

    dbg!(total_score);

    let rounds = contents
        .trim()
        .split("\n")
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| {
            (
                x[0].parse::<Move>().unwrap(),
                x[1].parse::<GameResult>().unwrap(),
            )
        })
        .collect::<Vec<(Move, GameResult)>>();

    let mut total_score = 0;

    for round in rounds {
        let my_move = get_move(&round);
        let result = get_winner((&round.0, &my_move));
        total_score += result.value() + my_move.score();
    }

    dbg!(total_score);

    Ok(())
}

fn get_move(round: &(Move, GameResult)) -> Move {
    match round {
        (Move::Rock, GameResult::Loss) => Move::Scissors,
        (Move::Rock, GameResult::Draw) => Move::Rock,
        (Move::Rock, GameResult::Win) => Move::Paper,
        (Move::Paper, GameResult::Loss) => Move::Rock,
        (Move::Paper, GameResult::Draw) => Move::Paper,
        (Move::Paper, GameResult::Win) => Move::Scissors,
        (Move::Scissors, GameResult::Loss) => Move::Paper,
        (Move::Scissors, GameResult::Draw) => Move::Scissors,
        (Move::Scissors, GameResult::Win) => Move::Rock,
    }
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> usize {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!(),
        })
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

impl FromStr for GameResult {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!(),
        })
    }
}

fn get_winner(round: (&Move, &Move)) -> GameResult {
    match round {
        (Move::Rock, Move::Rock) => GameResult::Draw,
        (Move::Rock, Move::Paper) => GameResult::Win,
        (Move::Rock, Move::Scissors) => GameResult::Loss,
        (Move::Paper, Move::Rock) => GameResult::Loss,
        (Move::Paper, Move::Paper) => GameResult::Draw,
        (Move::Paper, Move::Scissors) => GameResult::Win,
        (Move::Scissors, Move::Rock) => GameResult::Win,
        (Move::Scissors, Move::Paper) => GameResult::Loss,
        (Move::Scissors, Move::Scissors) => GameResult::Draw,
    }
}
