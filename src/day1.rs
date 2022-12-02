use crate::utils::read_lines;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Eq)]
struct ElfEntry(usize);

impl From<ElfEntry> for usize {
    fn from(elf: ElfEntry) -> usize {
        elf.0
    }
}

impl PartialOrd for ElfEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ElfEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialEq for ElfEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn run() -> std::io::Result<()> {
    let mut running_calories: Option<usize> = None;
    let mut elf_list: BinaryHeap<ElfEntry> = BinaryHeap::new();

    // assume valid input for now
    for line in read_lines("./data/day1.txt")? {
        let line = line?;
        if line.is_empty() {
            if running_calories.is_none() {
                continue;
            }
            let new_entry = ElfEntry(running_calories.unwrap());
            running_calories = None;
            push_into_heap_with_fixed_size(&mut elf_list, new_entry);
        } else {
            let parsed_line = line.parse::<usize>();
            let updated = running_calories.unwrap_or(0) + parsed_line.unwrap();
            running_calories = Some(updated);
        }
    }

    // if we hit EOF before newline, we need to flush again
    if running_calories.is_some() {
        let new_entry = ElfEntry(running_calories.unwrap());
        push_into_heap_with_fixed_size(&mut elf_list, new_entry);
    }

    let max_cals: usize = elf_list.peek().map_or(0, |x| (*x).into());
    println!("Max calories are {}", max_cals);

    let sum_of_three = elf_list
        .into_iter()
        .map(|x| x.into())
        .reduce(|acc, cal| acc + cal)
        .unwrap_or(0);
    println!("The sum of the highest three calories is {}", sum_of_three);

    Ok(())
}

fn push_into_heap_with_fixed_size<T>(heap: &mut BinaryHeap<T>, elem: T) -> ()
where
    T: Ord,
{
    // always push into heap, and if heap is too big then flush last out
    // todo: drain_sorted()
    heap.push(elem);
    if heap.len() > 3 {
        let mut tmp = vec![];
        for _ in 0..3 {
            tmp.push(heap.pop().unwrap());
        }
        heap.clear();
        for elem in tmp {
            heap.push(elem);
        }
    }
}
