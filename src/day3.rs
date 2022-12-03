use anyhow::anyhow;
use std::collections::HashSet;

pub fn run() -> anyhow::Result<()> {
    let content = include_str!("../data/day3.txt");
    let mut sum: usize = 0;
    for line in content.split_whitespace() {
        // split line in two halves, look for only repeat (hashmap?)
        // then calculate priority

        let split_point = line.len() / 2;
        let first_half = &line[0..split_point];
        let second_half = &line[split_point..];
        let repeated_char = find_repeat(first_half, second_half)?;
        let priority = calculate_priority(repeated_char)?;
        sum += priority as usize;
    }
    println!("sum of priorities is {sum}");
    Ok(())
}

fn calculate_priority(ch: char) -> anyhow::Result<u8> {
    if ch >= 'a' && ch <= 'z' {
        let score: u8 = (ch as u8) - ('a' as u8) + 1;
        Ok(score)
    } else if ch >= 'A' && ch <= 'Z' {
        let score: u8 = (ch as u8) - ('A' as u8) + 27;
        Ok(score)
    } else {
        Err(anyhow!("Invalid input"))
    }
}

fn find_repeat(fst: &str, snd: &str) -> anyhow::Result<char> {
    let mut seen: HashSet<char> = HashSet::new();

    for ch in fst.chars() {
        seen.insert(ch);
    }

    for ch in snd.chars() {
        if let Some(_) = seen.get(&ch) {
            return Ok(ch);
        }
    }

    Err(anyhow!(
        "No overlapping characters for first half {fst} and second half {snd}"
    ))
}
