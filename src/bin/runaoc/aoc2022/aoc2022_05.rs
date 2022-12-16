use std::collections::VecDeque;

pub struct Aoc2022_05 {
    crates: [VecDeque<char>; 9],
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl Aoc2022_05 {
    pub fn new() -> Self {
        Self {
            crates: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            instructions: vec![],
        }
    }
}

impl crate::Runner for Aoc2022_05 {
    fn name(&self) -> (usize, usize) {
        (2022, 05)
    }

    fn parse(&mut self) -> () {
        let file = "input/2022-05.txt";

        for line in aoc::read_linedata_from_file::<String>(file) {
            for (idx, ch) in line.chars().enumerate() {
                if ch == '[' {
                    // '[' will always be followed by a character so unwrap is fine here
                    self.crates[idx / 4].push_back(line.chars().nth(idx + 1).unwrap())
                }

                // save some time by breaking at the start of 'move'
                if ch == 'm' {
                    break;
                }
            }

            if line.starts_with("move") {
                let raw_instruction = line
                    .split(" ")
                    .filter(|w| w.parse::<usize>().is_ok())
                    .map(|w| w.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                self.instructions.push(Instruction {
                    num: raw_instruction[0],
                    from: raw_instruction[1],
                    to: raw_instruction[2],
                })
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {

        let mut crates = self.crates.clone();


        for instruction in self.instructions.iter() {
           
            for _ in 0..instruction.num {
                
                if let Some(content) = crates[instruction.from - 1].pop_front() {
                    crates[instruction.to - 1].push_front(content);
                }
            }
        }

        let mut res = String::with_capacity(9);

        for crte in crates.iter_mut() {
            if let Some(ch) = crte.pop_front() {
                res.push(ch);
            }
        }

        crate::output(res)
    }

    fn part2(&mut self) -> Vec<String> {
        
        let mut crates = self.crates.clone();
        
        for instruction in self.instructions.iter() {
           

            let mut stack = vec![];
            for _ in 0..instruction.num {
                

                if let Some(content) = crates[instruction.from - 1].pop_front() {
                    
                    
                    stack.push(content);
                    
                }
            }
            for ch in stack.iter().rev() {
                crates[instruction.to - 1].push_front(*ch);
            }
        }      

        let mut res = String::with_capacity(9);
        
        for crte in crates.iter_mut() {
            if let Some(ch) = crte.pop_front() {
                res.push(ch);
            }
        }       
        
        crate::output(res)
    }
}
