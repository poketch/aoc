pub struct Aoc2022_06 {
    buffer: Vec<char>,
}

impl Aoc2022_06 {
    pub fn new() -> Self {
        Self { buffer: vec![] }
    }
}

impl crate::Runner for Aoc2022_06 {
    fn name(&self) -> (usize, usize) {
        (2022, 06)
    }

    fn parse(&mut self) -> () {
        let file = "input/2022-06.txt";

        let content = aoc::read_linedata_from_file::<String>(file);

        self.buffer = content[0].as_str().chars().collect();
    }

    fn part1(&mut self) -> Vec<String> {
        for (idx, window) in self.buffer.as_slice().windows(4).enumerate() {
            let mut seen = std::collections::HashSet::new();

            for ch in window {
                if !seen.insert(ch) {
                    break;
                }

                if seen.len() == 4 {
                    return crate::output(idx + 4);
                }
            }
        }

        crate::output("unreachable")
    }
    
    fn part2(&mut self) -> Vec<String> {
        for (idx, window) in self.buffer.as_slice().windows(14).enumerate() {
            let mut seen = std::collections::HashSet::new();
            
            for ch in window {
                if !seen.insert(ch) {
                    break;
                }
                
                if seen.len() == 14 {
                    return crate::output(idx + 14);
                }
            }
        }
        
        crate::output("unreachable")
    }
}
