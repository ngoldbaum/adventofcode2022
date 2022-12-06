use regex::Regex;
use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    let (schematic, instructions) = contents.split_once("\n\n").unwrap();

    let stack_labels = schematic.lines().last().unwrap();

    let num_stacks: usize = stack_labels
        .split("  ")
        .map(|x| x.trim().parse().unwrap())
        .max()
        .unwrap();

    let schematic = schematic.strip_suffix(stack_labels).unwrap().trim_end();

    let crates = schematic
        .lines()
        .map(|x| {
            let c = x.chars().collect::<Vec<char>>();
            c.chunks(4)
                .map(|x| {
                    let x = x.to_vec();
                    if x.len() == 4 {
                        (&x[..3]).to_vec().iter().collect()
                    } else {
                        (&x[..]).to_vec().iter().collect()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut stack: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    for crate_row in crates {
        for (i, c) in crate_row.iter().enumerate() {
            if c == "   " {
                continue;
            }
            stack[i].push(c.chars().nth(1).unwrap())
        }
    }

    dbg!(&stack);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in instructions.lines() {
        dbg!(&stack);
        dbg!(line);
        let caps = re.captures(line).unwrap();
        let num_to_move: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let origin: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let origin = origin - 1;
        let destination: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        let destination = destination - 1;
        let orig_crates = &mut stack[origin];
        let move_crates = orig_crates.drain(..num_to_move).collect::<Vec<char>>();
        for c in move_crates {
            stack[destination].insert(0, c);
        }
    }

    let answer = &stack
        .iter()
        .map(|x| x.iter().next().unwrap())
        .collect::<String>();

    dbg!(answer);

    Ok(())
}
