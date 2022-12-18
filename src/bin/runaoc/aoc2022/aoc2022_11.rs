use std::collections::VecDeque;

pub struct Aoc2022_11 {
    monkeys: Vec<Monkey>,
}

#[derive(Debug, Clone)]
enum Op {
    Add(i32),
    Mul(i32),
    Sq,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Op,
    test_value: usize,
    send_true: usize, // number of the monkey to send on test True
    send_false: usize,
    inspections: u128,
}

impl Monkey {
    fn new(
        items: VecDeque<usize>,
        operation: Op,
        test_value: usize,
        send_true: usize,
        send_false: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test_value,
            send_true,
            send_false,
            inspections: 0,
        }
    }

    fn parse(input: Vec<String>) -> Self {
        // assmuning the input is in the format "items, Op, test, true, false"

        let items = &input[0];
        let operation = &input[1];
        let test = &input[2];
        let s_true = &input[3];
        let s_false = &input[4];

        let items = items
            .split(", ")
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

        // format: "Operation: new = old (+/ *) (num/old)"
        let raw_op = operation.split(" ").collect::<Vec<&str>>();
        let operation = match raw_op[raw_op.len() - 2] {
            "+" => Op::Add(raw_op.last().unwrap().parse::<i32>().unwrap()),

            "*" => match raw_op[raw_op.len() - 1] {
                "old" => Op::Sq,

                _ => Op::Mul(raw_op.last().unwrap().parse::<i32>().unwrap()),
            },

            _ => unreachable!("parsing op"),
        };

        // format: "Test: divisible by 19"
        let test_value = test.split(" ").last().unwrap().parse::<usize>().unwrap();

        // format: "If True: thow to monkey 2"
        let send_true = s_true.split(" ").last().unwrap().parse::<usize>().unwrap();

        // format: "If False: thow to monkey 2"
        let send_false = s_false.split(" ").last().unwrap().parse::<usize>().unwrap();

        Self::new(items, operation, test_value, send_true, send_false)
    }
}

impl Monkey {
    fn inspect(&mut self, worry_level: usize, panik: Option<usize>) -> usize {
        self.inspections += 1;
        let new_value = match self.operation {
            Op::Add(rhs) => ((self.items.pop_front().unwrap() as i32 + rhs) as usize),
            Op::Mul(rhs) => ((self.items.pop_front().unwrap() as i32 * rhs) as usize),
            Op::Sq => {
                let old = self.items.pop_front().unwrap();
                old * old
            }
        };

        match panik {
            Some(panic_level) => new_value % panic_level,
            None => new_value / worry_level,
        }
    }

    fn test(&self, worry: usize) -> bool {
        (worry) % self.test_value == 0
    }

    fn throw(&mut self, worry_level: usize, panik: Option<usize>) -> (usize, usize) {
        let new_worry = self.inspect(worry_level, panik);

        let to = if self.test(new_worry) {
            self.send_true
        } else {
            self.send_false
        };

        (to, new_worry)
    }

    fn receive(&mut self, item: usize) {
        self.items.push_back(item);
    }

    fn take_turn(&mut self, worry_level: usize, panik: Option<usize>) -> Option<(usize, usize)> {
        if self.items.len() < 1 {
            return None;
        }
        Some(self.throw(worry_level, panik))
    }
}

impl Aoc2022_11 {
    pub fn new() -> Self {
        Self { monkeys: vec![] }
    }

    fn round(monkeys: &mut Vec<Monkey>, worry_level: usize, panik: Option<usize>) {
        for idx in 0..monkeys.len() {
            for _ in 0..monkeys[idx].items.len() {
                let curr = &mut monkeys[idx];
                let info = curr.take_turn(worry_level, panik);

                if info.is_some() {
                    let (to, item) = info.unwrap();
                    monkeys[to].receive(item);
                }
            }
        }
    }
}

impl crate::Runner for Aoc2022_11 {
    fn name(&self) -> (usize, usize) {
        (2022, 11)
    }

    fn parse(&mut self) -> () {
        // let file = "test.txt";
        // let mut content = aoc::read_packets_from_test_file::<String>(file);

        let file = "input/2022-11.txt";
        let mut content = aoc::read_packets_from_file::<String>(file);

        let raw_monkeys = content
            .iter_mut()
            .map(|v| {
                v.remove(0);
                v.iter()
                    .map(|s| {
                        let (_, tail) = s.split_once(": ").unwrap();
                        tail.to_string()
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        self.monkeys = raw_monkeys
            .iter()
            .map(|v| Monkey::parse(v.clone()))
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut monkeys = self.monkeys.clone();
        const ROUNDS: usize = 20;

        for _ in 0..ROUNDS {
            Aoc2022_11::round(&mut monkeys, 3, None);
        }

        let mut m_business = monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<u128>>();

        m_business.sort();
        let m_business = m_business.iter().rev().take(2).product::<u128>();

        crate::output(m_business)
    }

    fn part2(&mut self) -> Vec<String> {
        
        let mut monkeys = self.monkeys.clone();
        const ROUNDS: usize = 10_000;

        let panic_level = self.monkeys.iter().map(|m| m.test_value).product::<usize>();

        for _ in 0..ROUNDS {
            Aoc2022_11::round(&mut monkeys, 1, Some(panic_level));
        }

        let mut m_business = monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<u128>>();

        m_business.sort();
        let m_business = m_business.iter().rev().take(2).product::<u128>();

        crate::output(m_business)
    }
}
