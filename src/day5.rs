use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
struct ElfCrate {
    val: char,
}

type Stack = Vec<ElfCrate>;

#[derive(Debug, Default, Copy, Clone)]
struct Instruction {
    quantity: u8, // must be big enough?
    src: usize,
    to: usize,
}

impl Instruction {
    pub fn new(quantity: u8, src: usize, to: usize) -> Self {
        Self { quantity, src, to }
    }

    pub fn quantity(&self) -> u8 {
        self.quantity
    }
    pub fn src(&self) -> usize {
        self.src
    }
    pub fn to(&self) -> usize {
        self.to
    }

    pub fn parse_instruction(input: &str) -> anyhow::Result<Vec<Instruction>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let mut vec = vec![];
        let captures = RE.captures_iter(input);
        for cap in captures {
            let ins = Instruction::new(cap[1].parse()?, cap[2].parse()?, cap[3].parse()?);
            vec.push(ins);
        }

        Ok(vec)
    }

    pub fn apply_instructions(instructions: &Vec<Instruction>, crates: &mut Vec<Stack>) {
        for instruction in instructions.iter() {
            /* Part 1

            for _ in 0..instruction.quantity() {
                let from_crate = crates
                    .get_mut(instruction.src() - 1).expect("Invalid instruction src").pop();
                if let Some(c) = from_crate {
                    let to = crates
                    .get_mut(instruction.to() - 1)
                    .expect("Got invalid 'to' for instruction");

                    to.push(c);
                }
            }
            */
            Self::apply_instruction(&instruction, crates);
        }
    }

    fn apply_instruction(instruction: &Instruction, crates: &mut Vec<Stack>) {
        let from_stack = crates
            .get_mut(instruction.src() - 1)
            .expect("stack index from instruction does not exist");

        let mut crates_to_move = from_stack
            .drain((from_stack.len() - instruction.quantity() as usize)..)
            .collect_vec();

        let to_stack = crates
            .get_mut(instruction.to() - 1)
            .expect("stack index from instruction does not exist");

        to_stack.append(&mut crates_to_move);
    }
}

impl ElfCrate {
    pub fn from(s: char) -> Self {
        Self { val: s }
    }

    pub fn val(&self) -> char {
        self.val
    }

    pub fn parse_crates(input: &str) -> Vec<Stack> {
        // we want to create some vec with appropriate capacity
        let mut lines = input.lines().peekable();
        let number_of_stacks = (lines.peek().unwrap().len() + 1) / 4;
        let mut vec: Vec<Stack> = vec![Stack::new(); number_of_stacks];

        for line in input.lines() {
            if let Some('1') = line.chars().nth(1) {
                break;
            }

            for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
                if let Some('[') = chunk.next() {
                    let crate_val = chunk
                        .next()
                        .expect("should have input value after opening [");
                    vec.get_mut(idx)
                        .map(|stack| stack.insert(0, ElfCrate::from(crate_val)));
                }
            }
        }
        vec
    }
}

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../data/day5.txt");
    let mut vec = ElfCrate::parse_crates(input);
    let instructions = Instruction::parse_instruction(input)?;
    Instruction::apply_instructions(&instructions, &mut vec);

    let top = vec
        .iter()
        .map(|s| match s.last() {
            Some(c) => c.val(),
            None => ' ',
        })
        .join("");
    println!("{top}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_runs() {
        assert!(true)
    }

    fn it_parses_crates() {
        let test_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
    }
}
