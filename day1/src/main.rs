use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone, Default)]
struct ElfEntry(usize);

fn main() -> std::io::Result<()> {
    let mut running_calories: Option<usize> = None;
    let mut max_elf = ElfEntry::default();
    let mut elf_list: Vec<ElfEntry> = vec![];

    // assume valid input for now
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let line = line?;
            if line.is_empty() {
                let new_entry = ElfEntry(running_calories.unwrap_or(0));
                if new_entry.0 > max_elf.0 {
                    max_elf = new_entry;
                }
                running_calories = None;
                elf_list.push(new_entry);
            } else {
                let parsed_line = line.parse::<usize>();
                if parsed_line.is_err() {
                    println!(
                        "Got error for parsed line {} and got {:?}",
                        line, parsed_line
                    );
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Invalid err",
                    ));
                }
                let updated = running_calories.unwrap_or(0) + parsed_line.unwrap();
                running_calories = Some(updated);
            }
        }
    }

    // if we hit EOF before newline, we need to flush again
    if running_calories.is_some() {
        let new_entry = ElfEntry(running_calories.unwrap());
        if new_entry.0 > max_elf.0 {
            max_elf = new_entry;
        }
        elf_list.push(new_entry);
    }

    println!("Max calories are {}", max_elf.0);

    elf_list.sort_by(|a, b| b.0.cmp(&a.0));
    let sum_of_three = elf_list
        .iter()
        .take(3)
        .map(|elf| elf.0)
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
