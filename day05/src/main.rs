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

    let mut orig_stack: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    for crate_row in crates {
        for (i, c) in crate_row.iter().enumerate() {
            if c == "   " {
                continue;
            }
            orig_stack[i].push(c.chars().nth(1).unwrap())
        }
    }

    dbg!(solve(&mut orig_stack.clone(), instructions, false)?);
    dbg!(solve(&mut orig_stack.clone(), instructions, true)?);

    Ok(())
}

fn solve(stack: &mut Vec<Vec<char>>, instructions: &str, reverse: bool) -> Result<String> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;

    for line in instructions.lines() {
        let caps = re.captures(line).unwrap();
        let num_to_move: usize = caps.get(1).unwrap().as_str().parse()?;
        let origin: usize = caps.get(2).unwrap().as_str().parse()?;
        let destination: usize = caps.get(3).unwrap().as_str().parse()?;
        let orig_crates = &mut stack[origin - 1];
        let mut move_crates = orig_crates.drain(..num_to_move).collect::<Vec<char>>();
        if reverse {
            move_crates.reverse();
        }
        for c in move_crates {
            stack[destination - 1].insert(0, c);
        }
    }

    let answer = stack
        .iter()
        .map(|x| x.iter().next().unwrap())
        .collect::<String>();

    Ok(answer)
}
