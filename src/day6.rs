use itertools::Itertools;
use std::collections::HashMap;

fn run_impl(input: &str, window_size: usize) -> Option<usize> {
    let char_vec = input.chars().collect_vec();

    let first_enumeration = char_vec
        .windows(window_size)
        .enumerate()
        .find(|(_idx, window)| {
            // return if there's no repeat in the window
            let mapping = window.into_iter().fold(HashMap::new(), |mut acc, x| {
                acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
                acc
            });
            mapping.values().all(|&v| v == 1)
        });

    first_enumeration.map(|(idx, _)| idx + window_size)
}

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../data/day6.txt");
    let end_of_window_idx = run_impl(input, 4).ok_or_else(|| anyhow::anyhow!("No valid window"))?;
    println!("End of window at {end_of_window_idx}");
    let end_of_message_idx =
        run_impl(input, 14).ok_or_else(|| anyhow::anyhow!("No valid message"))?;
    println!("End of message at {end_of_message_idx}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day6::run_impl;
    #[test]
    fn it_finds_4_idx_window() {
        assert_eq!(Some(5), run_impl("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    }

    #[test]
    fn it_finds_14_idx_window() {
    assert_eq!(Some(23), run_impl("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    }

    #[test]
    fn it_returns_none_for_no_valid_window() {
        assert_eq!(None, run_impl("babbebbdbabbe", 4));
    }

    #[test]
    fn it_returns_none_for_empty_str() {
        assert_eq!(None, run_impl("", 4));
    }

    #[test]
    fn it_returns_none_for_giant_window() {
        assert_eq!(None, run_impl("abcdefg", 20));
    }
}
