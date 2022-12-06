use std::collections::HashSet;
use std::error;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = read_to_string("input")?;

    dbg!(get_marker_index(contents.trim(), 4));
    dbg!(get_marker_index(contents.trim(), 14));

    Ok(())
}

fn get_marker_index(msg: &str, window_size: usize) -> usize {
    let mut i: usize = 0;
    for w in msg.chars().collect::<Vec<char>>().windows(window_size) {
        i += 1;
        let unique_chars: HashSet<&char> = HashSet::from_iter(w);
        if unique_chars.len() == window_size {
            break;
        }
    }

    i + window_size - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_marker() {
        let test_strings: HashMap<&str, (usize, usize)> = HashMap::from([
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", (7, 19)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", (5, 23)),
            ("nppdvjthqldpwncqszvftbrmjlhg", (6, 23)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", (10, 29)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", (11, 26)),
        ]);

        for (msg, (i, j)) in &test_strings {
            assert_eq!(get_marker_index(msg, 4), *i);
            assert_eq!(get_marker_index(msg, 14), *j);
        }
    }
}
