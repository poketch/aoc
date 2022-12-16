use std::collections::HashSet;

#[derive(Default)]
pub struct Aoc2022_09 {
    moves: Vec<(Direction, i32)>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Self::UP,
            "D" => Self::DOWN,
            "L" => Self::LEFT,
            "R" => Self::RIGHT,
            _ => panic!("Bad Direction: {s}"),
        }
    }
}

struct Snake {
    segments: Vec<(i32, i32)>, // head is 0
    tail_pos: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Snake {

    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn new(len: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        
        Self {
            segments: vec![(0, 0); len],
            tail_pos: (0, 0),
            visited
        }
    }

    fn do_move(&mut self, dir: &Direction) {
        
        //move the head,
        let command = Self::DIR[*dir as usize];
        self.segments[0].0 += command.0;
        self.segments[0].1 += command.1;

        //determine if the tail has to move

        for index in 1..self.segments.len() {

            let curr = self.segments[index];
            let prev = self.segments[index - 1];

            let row_diff = prev.0 - curr.0;
            let col_diff = prev.1 - curr.1;

            if row_diff.abs() > 1 || col_diff.abs() > 1 {

                self.segments[index].0 += row_diff.signum();
                self.segments[index].1 += col_diff.signum();
            }
        }

        //record the tail position.
        let tail = self.tail();
        if tail != self.tail_pos {
            self.visited.insert(tail);
            self.tail_pos = tail;
        }

    }

    fn tail(&self) -> (i32, i32) {
        *self.segments.last().unwrap()
    }

    fn unique_visits(&self) -> usize {
        self.visited.len()
    }


}

impl Aoc2022_09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl crate::Runner for Aoc2022_09 {
    fn name(&self) -> (usize, usize) {
        (2022, 09)
    }

    fn parse(&mut self) -> () {
        let file = "input/2022-09.txt";

        self.moves = aoc::read_linedata_from_file::<String>(file).iter().map(|m| {
        
            let (dir, dist) = m.split_once(" ").unwrap();

            (Direction::parse(dir), dist.parse::<i32>().unwrap())
        }).collect();

    }

    fn part1(&mut self) -> Vec<String> {
        
        let mut snake = Snake::new(2);
        
        for (dir, amount) in self.moves.iter() {

            for _ in 0..*amount {

                snake.do_move(&dir);
            }

        }
        
        crate::output(snake.unique_visits())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut snake = Snake::new(10);
        
        for (dir, amount) in self.moves.iter() {

            for _ in 0..*amount {

                snake.do_move(&dir);
            }

        }
        
        crate::output(snake.unique_visits())
    }
}
