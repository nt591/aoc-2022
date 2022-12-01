use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone)]
struct ElfEntry(usize);

impl From<ElfEntry> for usize {
    fn from(elf: ElfEntry) -> usize {
        elf.0
    }
}

pub fn run() -> std::io::Result<()> {
    let mut running_calories: Option<usize> = None;
    let mut elf_list: Vec<ElfEntry> = vec![];

    // assume valid input for now
    for line in read_lines("./data/day1.txt")? {
        let line = line?;
        if line.is_empty() {
            if running_calories.is_none() {
                continue;
            }
            let new_entry = ElfEntry(running_calories.unwrap());
            running_calories = None;
            elf_list.push(new_entry);
        } else {
            let parsed_line = line.parse::<usize>();
            let updated = running_calories.unwrap_or(0) + parsed_line.unwrap();
            running_calories = Some(updated);
        }
    }

    // if we hit EOF before newline, we need to flush again
    if running_calories.is_some() {
        let new_entry = ElfEntry(running_calories.unwrap());
        elf_list.push(new_entry);
    }

    elf_list.sort_by(|a, b| b.0.cmp(&a.0));
    let max_cals = elf_list.get(0).map_or(0, |x| (*x).into());
    println!("Max calories are {}", max_cals);

    let sum_of_three = elf_list
        .into_iter()
        .take(3)
        .map(|x| x.into())
        .reduce(|acc, cal| acc + cal)
        .unwrap_or(0);
    println!("The sum of the highest three calories is {}", sum_of_three);

    Ok(())
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<BufReader<File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
