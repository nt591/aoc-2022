use anyhow::anyhow;
use itertools::Itertools;
use std::collections::HashSet;

pub fn run() -> anyhow::Result<()> {
    let content = include_str!("../data/day3.txt");
    let mut sum: usize = 0;
    for mut group in &content.split_whitespace().collect::<Vec<&str>>().into_iter().chunks(3) {
        let fst = group.next().unwrap();
        let snd = group.next().unwrap();
        let thd = group.next().unwrap();
        let repeated_char = find_repeat(fst, snd, thd)?;
        let priority = calculate_priority(&repeated_char)?;
        sum += priority as usize;
    }
    println!("sum of priorities is {sum}");
    Ok(())
}

fn calculate_priority(ch: &char) -> anyhow::Result<u8> {
    if ch >= &'a' && ch <= &'z' {
        let score: u8 = (*ch as u8) - ('a' as u8) + 1;
        Ok(score)
    } else if ch >= &'A' && ch <= &'Z' {
        let score: u8 = (*ch as u8) - ('A' as u8) + 27;
        Ok(score)
    } else {
        Err(anyhow!("Invalid input"))
    }
}

fn find_repeat(fst: &str, snd: &str, thd: &str) -> anyhow::Result<char> {
    let mut fst_chars: HashSet<char> = HashSet::new();
    for ch in fst.chars() {
        fst_chars.insert(ch);
    }
    let mut snd_chars: HashSet<char> = HashSet::new();
    for ch in snd.chars() {
        snd_chars.insert(ch);
    }
    let mut thd_chars: HashSet<char> = HashSet::new();
    for ch in thd.chars() {
        thd_chars.insert(ch);
    }
    let intersection_fst_and_second: HashSet<char> = fst_chars.intersection(&snd_chars).map(|c| *c).collect();
    let intersection_all: HashSet<&char> = intersection_fst_and_second.intersection(&thd_chars).collect();

    match intersection_all.len() {
        1 => Ok(*intersection_all.into_iter().next().unwrap()),
        _ => Err(anyhow!(
            "No overlapping characters for in grouping {fst} {snd} {thd}"
        )),
    }
}
