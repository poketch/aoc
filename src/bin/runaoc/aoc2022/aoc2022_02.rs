pub struct Aoc2022_02 {
    rounds: Vec<Vec<String>>,
}

#[derive(Debug)]
enum Move {
    // Col 1 map: A -> Rock, B -> Paper, C -> Scissors
    // Col 2 map: X -> Rock, Y -> Paper, Z -> Scissors
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> u32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Round {
    Win,
    Lose,
    Draw,
}

impl Round {
    fn value(&self) -> u32 {
        match *self {
            Round::Win => 6,
            Round::Lose => 0,
            Round::Draw => 3,
        }
    }
}

impl Aoc2022_02 {
    pub fn new() -> Self {
        Self { rounds: vec![] }
    }

    fn calc_rps(opp: &Move, my: &Move) -> u32 {
        let round = match my {
            Move::Rock => match opp {
                Move::Rock => Round::Draw,
                Move::Paper => Round::Lose,
                Move::Scissors => Round::Win,
            },
            Move::Paper => match opp {
                Move::Rock => Round::Win,
                Move::Paper => Round::Draw,
                Move::Scissors => Round::Lose,
            },
            Move::Scissors => match opp {
                Move::Rock => Round::Lose,
                Move::Paper => Round::Win,
                Move::Scissors => Round::Draw,
            },
        };

        round.value() + my.value()
    }

    fn predict_rps(opp: &Move, result: &Round) -> u32 {
        let my = match result {
            Round::Win => match opp {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Round::Lose => match opp {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Round::Draw => match opp {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
        };

        my.value() + result.value()
    }
}

impl crate::Runner for Aoc2022_02 {
    fn name(&self) -> (usize, usize) {
        (2022, 2)
    }

    fn parse(&mut self) -> () {
        let file = "input/2022-02.txt";

        let content: Vec<String> = aoc::read_linedata_from_file(file);

        self.rounds = content
            .iter()
            .map(|s| s.split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
    }

    fn part1(&mut self) -> Vec<String> {
        let res = self
            .rounds
            .iter()
            .map(|r| {
                r.iter()
                    .map(|m| match m.as_str() {
                        "A" | "X" => Move::Rock,
                        "B" | "Y" => Move::Paper,
                        "C" | "Z" => Move::Scissors,
                        _ => panic!("Invalid Move input"),
                    })
                    .collect::<Vec<Move>>()
            })
            .collect::<Vec<Vec<Move>>>();

        let ans = res
            .iter()
            .map(|r| Self::calc_rps(&r[0], &r[1]))
            .sum::<u32>();

        crate::output(ans)
    }

    fn part2(&mut self) -> Vec<String> {
        
        let res: Vec<(Move, Round)> = self.rounds
        .iter()
        .map(|round| {
            ( match round[0].as_str() {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!("invalid"),
            },
            match round[1].as_str() {
                "X" => Round::Lose,
                "Y" => Round::Draw,
                "Z" => Round::Win,
                _ => panic!("invalid"),
            })
        })
        .collect();
        
        let ans = res
        .iter()
        .map(|round| Self::predict_rps(&round.0, &round.1))
        .sum::<u32>();

        crate::output(ans)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let input = "A B\nB X\nC Z";

        let content: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();

        let rounds: Vec<Vec<Move>> = content
            .iter()
            .map(|s| {
                s.split(" ")
                    .map(|m| match m {
                        "A" | "X" => Move::Rock,
                        "B" | "Y" => Move::Paper,
                        "C" | "Z" => Move::Scissors,
                        _ => panic!("Invalid Move input"),
                    })
                    .collect::<Vec<Move>>()
            })
            .collect();

        let res: Vec<u32> = rounds
            .iter()
            .map(|r| Aoc2022_02::calc_rps(&r[0], &r[1]))
            .collect();

        assert_eq!(res.iter().sum::<u32>(), 15)
    }
}
