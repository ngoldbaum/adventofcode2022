use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input")?;

    let mut values = contents
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .into_iter()
                .sum()
        })
        .collect::<Vec<i64>>();

    values.sort();

    let max_val = values.iter().max().unwrap();

    dbg!(max_val);

    dbg!(values[values.len() - 3..].iter().sum::<i64>());

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
