use std::collections::HashSet;
use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

static ASCII: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let total_priority = contents
        .lines()
        .map(|x| {
            let num_items = x.len();
            let c = (&x[0..num_items / 2], &x[num_items / 2..]);
            let c: (HashSet<char>, HashSet<char>) = (c.0.chars().collect(), c.1.chars().collect());
            let int = c.0.intersection(&c.1).copied();
            let int = int.last().unwrap();
            ASCII.chars().position(|x| x == int).unwrap() + 1
        })
        .sum::<usize>();

    dbg!(total_priority);

    let total_priority = contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .into_iter()
        .map(|x| {
            let s: (HashSet<char>, HashSet<char>, HashSet<char>) = (
                x[0].chars().collect(),
                x[1].chars().collect(),
                x[2].chars().collect(),
            );
            let int = s.0.intersection(&s.1).copied().collect::<HashSet<char>>();
            let int = int.intersection(&s.2).copied().last().unwrap();
            ASCII.chars().position(|x| x == int).unwrap() + 1
        })
        .sum::<usize>();

    dbg!(total_priority);

    Ok(())
}
