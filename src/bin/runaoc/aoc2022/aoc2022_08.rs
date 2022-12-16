use std::collections::HashSet;


#[derive(Default)]
pub struct Aoc2022_08 {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Aoc2022_08 {
    pub fn new() -> Self {
        Self::default()
    }

    const UP: (i32, i32) = (-1, 0);
    const DOWN: (i32, i32) = (1, 0);
    const LEFT: (i32, i32) = (0, -1);
    const RIGHT: (i32, i32) = (0, 1);
}

impl crate::Runner for Aoc2022_08 {
    fn name(&self) -> (usize, usize) {
        (2022, 08)
    }

    fn parse(&mut self) -> () {
        let file = "input/2022-08.txt";

        self.grid = aoc::read_linedata_from_file::<String>(file)
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        self.width = self.grid[0].len() as i32;
        self.height = self.grid.len() as i32;

    }

    fn part1(&mut self) -> Vec<String> {
        
        // Instead of looking from each position, look down each direction

        let mut total = HashSet::new(); // Hash set blocks the dups

        for (start, step, direction) in vec![
            ((0,0),Aoc2022_08::DOWN,Aoc2022_08::RIGHT),
            ((0,0), Aoc2022_08::RIGHT, Aoc2022_08::DOWN),
            ((self.height - 1, self.width - 1 ), Aoc2022_08::UP, Aoc2022_08::LEFT),
            ((self.height - 1, self.width - 1 ), Aoc2022_08::LEFT, Aoc2022_08::UP)]
            {

                let mut walker = start;

                //while walker is within bounds
                while walker.0 >= 0 && walker.0 < self.width && walker.1 >= 0 && walker.1 < self.height {

                    let (mut row, mut col) = walker;
                    let mut tallest = self.grid[row as usize][col as usize]; //setting the edge tree to teh tallest in the current set

                    total.insert((row,col)); // adding the edge tree

                    while tallest < '9' { //keep looking down that row until you find a tree that is too tall
                        
                        row += direction.0;
                        col += direction.1;

                        if row < 0 || row >= self.width || col < 0 || col >= self.height {
                            break;
                        }

                        let tree = self.grid[row as usize][col as usize];
                        if tree > tallest {
                            total.insert((row,col));
                            tallest = tree;
                        }
                    }

                    //once you're done looking down that row move the walker
                    walker.0 += step.0;
                    walker.1 += step.1;
                }
            }

        crate::output(total.len())
    }

    fn part2(&mut self) -> Vec<String> {


        let mut max_score = 1;

        for row in 1..self.height - 1 {
            for col in 1..self.width - 1 {

                let mut score = 1;
                
                for step in [Aoc2022_08::UP, Aoc2022_08::DOWN, Aoc2022_08::LEFT, Aoc2022_08::RIGHT] {

                    let mut walker = (row, col);

                    let my_height = self.grid[row as usize][col as usize];

                    walker.0 += step.0;
                    walker.1 += step.1;

                    let mut count = 0; // you will always see one tree

                    while walker.0 >= 0 && walker.0 < self.height && walker.1 >= 0 && walker.1 < self.width {

                        count += 1; 
                        
                        if self.grid[walker.0 as usize][walker.1 as usize] >= my_height {
                            break;
                        }
                        
                        walker.0 += step.0;
                        walker.1 += step.1;
                    }

                    score *= count;
                    
                }

                max_score = max_score.max(score);
                
            }
        }

        crate::output(max_score)
    }
}
