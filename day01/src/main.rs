use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let mut values = contents
        .trim()
        .split("\n\n")
        .map(|x| x.split("\n").map(|y| y.parse::<i64>().unwrap()).sum())
        .collect::<Vec<i64>>();

    values.sort();

    dbg!(values[values.len() - 1]);

    dbg!(values[values.len() - 3..].iter().sum::<i64>());

    Ok(())
}
