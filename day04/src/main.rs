use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let ranges: Vec<((usize, usize), (usize, usize))> = contents
        .trim()
        .lines()
        .map(|x| {
            let a = x.split_once(',').unwrap();
            (get_range(a.0), get_range(a.1))
        })
        .collect();

    let contains: usize = ranges
        .iter()
        .map(|a| (contains(a.0, a.1) || contains(a.1, a.0)) as usize)
        .sum();

    dbg!(contains);

    let overlaps: usize = ranges.iter().map(|a| overlaps(a.0, a.1) as usize).sum();

    dbg!(overlaps);

    Ok(())
}

fn get_range(range: &str) -> (usize, usize) {
    let range: (&str, &str) = range.split_once('-').unwrap();

    (range.0.parse().unwrap(), range.1.parse().unwrap())
}

fn contains(r1: (usize, usize), r2: (usize, usize)) -> bool {
    r2.0 >= r1.0 && r2.1 <= r1.1
}

fn overlaps(r1: (usize, usize), r2: (usize, usize)) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}
