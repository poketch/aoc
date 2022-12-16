pub struct Aoc2022_10 {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NOOP,
    ADD(i32),
}
struct CPU {
    reg_x: i32,
    clock: i32,
    signals: Vec<i32>,
}

impl CPU {
    fn new() -> Self {
        Self {
            reg_x: 1,
            clock: 0,
            signals: vec![],
        }
    }

    fn tick(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOOP => {
                self.clock += 1;
                if (self.clock - 20) % 40 == 0 {
                    self.signals.push(self.clock * self.reg_x);
                }
            }

            Instruction::ADD(value) => {
                for _ in 0..2 {
                    self.clock += 1;

                    if (self.clock - 20) % 40 == 0 {
                        self.signals.push(self.clock * self.reg_x);
                    }
                }

                self.reg_x += value;
            }
        }
    }
}

struct CRT {
    cpu: CPU,
    sprite: i32,
    draw_cycle: i32,
}

impl CRT {
    fn new() -> Self {
        Self {
            cpu: CPU::new(),
            sprite: 1,
            draw_cycle: 0,
        }
    }

    fn sprite_pos(&self) -> [i32; 3] {
        [self.sprite - 1, self.sprite, self.sprite + 1]
    }

    fn move_sprite(&mut self, loc: i32) {
        self.sprite = loc; // instructions don't say what happens if it goes out of bounds
    }

    fn draw(&mut self, instruction: Instruction) {
        self.cpu.tick(instruction);

        let catch_up_cycles = self.cpu.clock - self.draw_cycle;
        for _ in 0..catch_up_cycles {

            /* 
                Here the CRT spends time "catching up" to the cpu clock by 
                filling the blanks in the number of steps taken by the cpu 
                since the sprite is not moved until the end of the tick
            */

            if self.sprite_pos().contains(&(self.draw_cycle % 40)) {
                print!("#");
            } else {
                print!(".");
            }

            self.draw_cycle += 1;

            if self.draw_cycle % 40 == 0 {
                println!();
            }

        }
        self.move_sprite(self.cpu.reg_x);
    }
}

impl Aoc2022_10 {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
        }
    }
}

impl crate::Runner for Aoc2022_10 {
    fn name(&self) -> (usize, usize) {
        (2022, 10)
    }

    fn parse(&mut self) -> () {
        // let file = "test.txt";
        // let content = aoc::read_linedata_from_test_file::<String>(file);

        let file = "input/2022-10.txt";
        let content = aoc::read_linedata_from_file::<String>(file);

        for line in content.iter() {
            // line will always have one char atleast if file is properly parsed unwrap is fine here
            let command = match line.chars().next().unwrap() {
                'a' => {
                    let (_, amt) = line.split_once(" ").unwrap();
                    Instruction::ADD(amt.parse::<i32>().unwrap())
                }

                'n' => Instruction::NOOP,

                _ => panic!("bad input {line}"),
            };

            self.instructions.push(command);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut cpu = CPU::new();

        for instruction in self.instructions.iter() {
            cpu.tick(*instruction);
        }

        crate::output(cpu.signals.iter().sum::<i32>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut screen = CRT::new();

        for instruction in self.instructions.iter() {
            screen.draw(*instruction);
        }

        crate::output("Answer above")
    }
}
