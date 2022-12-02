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

impl RockPaperScissors {
    pub fn won(opp: RockPaperScissors, me: RockPaperScissors) -> Outcome {
        use RockPaperScissors::*;
        match (opp, me) {
            (opp, me) if opp == Rock && me == Scissors => Outcome::Lose,
            (opp, me) if opp == Scissors && me == Rock => Outcome::Win,
            (opp, me) => {
                let opp_score: usize = opp.into();
                let my_score: usize = me.into();
                if my_score > opp_score {
                    Outcome::Win
                } else if my_score < opp_score {
                    Outcome::Lose
                } else {
                    Outcome::Draw
                }
            }
        }
    }
    fn new(val: &str) -> Option<Self> {
        use RockPaperScissors::*;
        match val {
            "A" => Some(Rock),
            "B" => Some(Paper),
            "C" => Some(Scissors),
            "X" => Some(Rock),
            "Y" => Some(Paper),
            "Z" => Some(Scissors),
            _ => None,
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
        let results = line
            .split(" ")
            .filter_map(RockPaperScissors::new)
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

fn _test1() -> anyhow::Result<()> {
    let mut score: usize = 0;
    for line in read_lines("./data/day2.txt")? {
        let line = line?;
        let results = line
            .split(" ")
            .filter_map(RockPaperScissors::new)
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