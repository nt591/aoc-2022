use crate::utils::read_lines;

#[derive(PartialEq, Copy, Clone, Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn new(val: &str) -> Self {
        match val {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!(),
        }
    }
}

impl RockPaperScissors {
    fn loses_to(&self) -> Self {
        use RockPaperScissors::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn wins_against(&self) -> Self {
        use RockPaperScissors::*;
        match self {
            Rock => Scissors,
            Scissors => Paper,
            Paper => Rock,
        }
    }

    #[allow(dead_code)]
    pub fn won(opp: Self, me: Self) -> Outcome {
        if me.loses_to() == opp {
            Outcome::Lose
        } else if me.wins_against() == opp {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }

    pub fn new(val: &str) -> Self {
        use RockPaperScissors::*;
        match val {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!(),
        }
    }
}

impl From<RockPaperScissors> for usize {
    fn from(val: RockPaperScissors) -> Self {
        use RockPaperScissors::*;
        match val {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl From<Outcome> for usize {
    fn from(val: Outcome) -> Self {
        use Outcome::*;

        match val {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let mut score: usize = 0;
    for line in read_lines("./data/day2.txt")? {
        let line = line?;
        let mut results = line.split_whitespace();

        let opponent = RockPaperScissors::new(results.next().unwrap());
        let outcome = Outcome::new(results.next().unwrap());
        let myself = match outcome {
            Outcome::Win => opponent.loses_to(),
            Outcome::Draw => opponent,
            Outcome::Lose => opponent.wins_against(),
        };
        let outcome_score: usize = outcome.into();
        let my_score: usize = myself.into();
        score += my_score;
        score += outcome_score;
    }
    println!("My total score: {}", score);
    Ok(())
}

fn _test1() -> anyhow::Result<()> {
    let mut score: usize = 0;
    for line in read_lines("./data/day2.txt")? {
        let line = line?;
        let results = line
            .split(" ")
            .map(RockPaperScissors::new)
            .take(2)
            .collect::<Vec<RockPaperScissors>>();
        debug_assert!(results.len() == 2);

        let opponent = results[0];
        let myself = results[1];
        let my_score: usize = myself.into();
        let outcome_score: usize = (RockPaperScissors::won(opponent, myself)).into();
        score += my_score;
        score += outcome_score;
    }
    println!("My total score: {}", score);
    Ok(())
}
