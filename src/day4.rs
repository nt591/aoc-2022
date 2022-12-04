use anyhow::anyhow;
use itertools::Itertools;

#[derive(Eq, PartialEq)]
struct ElfRange {
    start: usize,
    end: usize,
}

impl ElfRange {
    fn try_from_input(input: &str) -> anyhow::Result<Self> {
        let mut split = input.split("-");
        let start = split.next().expect("No first element");
        let end = split.next().expect("No second element");
        if let Some(_) = split.next() {
            return Err(anyhow!("Invalid inputs from {input}"));
        }

        let start = start.parse::<usize>()?;
        let end = end.parse::<usize>()?;
        Ok(ElfRange { start, end })
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        // self.fully_contains(other) || other.fully_contains(&self)
        let first = if self.start < other.start {
            self
        } else {
            other
        };
        let second = if first == self { other } else { self };

        first.end >= second.start
    }

    #[allow(dead_code)]
    fn fully_contains(&self, other: &Self) -> bool {
        // fully contain means my start and end consume other
        self.start <= other.start && self.end >= other.end
    }
}

pub fn run() -> anyhow::Result<()> {
    let text = include_str!("../data/day4.txt");
    let parsed = text
        .lines()
        .map(|l| l.split(","))
        .flat_map(|v| v.map(ElfRange::try_from_input))
        .collect::<Vec<Result<ElfRange, _>>>();
    let mut sum = 0;
    for mut chunk in &parsed.into_iter().chunks(2) {
        let left = chunk
            .next()
            .unwrap_or_else(|| Err(anyhow!("Invalid inputs")))?;
        let right = chunk
            .next()
            .unwrap_or_else(|| Err(anyhow!("Invalid inputs")))?;
        sum += left.overlaps_with(&right) as usize;
    }
    println!("Sum of overlaps is {sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day4::ElfRange;
    #[test]
    fn it_finds_overlaps_in_range() {
        let left = ElfRange { start: 6, end: 6 };
        let right = ElfRange { start: 4, end: 6 };
        assert!(left.overlaps_with(&right))
    }

    #[test]
    fn it_rejects_missing_overlap() {
        let left = ElfRange { start: 2, end: 3 };
        let right = ElfRange { start: 4, end: 5 };
        assert_eq!(false, left.overlaps_with(&right))
    }

    #[test]
    fn it_rejects_touching_but_not_containing_boundaries() {
        let left = ElfRange { start: 5, end: 7 };
        let right = ElfRange { start: 7, end: 9 };
        assert_eq!(false, left.overlaps_with(&right))
    }
}
