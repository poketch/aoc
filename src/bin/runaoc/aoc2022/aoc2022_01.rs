pub struct Aoc2022_01 {
    sums: Vec<u32>,
}

impl Aoc2022_01 {
    pub fn new() -> Self {
        Self { sums: vec![] }
    }
}

impl crate::Runner for Aoc2022_01 {
    fn name(&self) -> (usize, usize) {
        (2022, 0x01)
    }

    fn parse(&mut self) -> () {
        
        let file_name = "input/2022-01.txt"; // file contains groups of numbers
        let packets = aoc::read_packets_from_file(file_name);

        // for both parts we're only interested in the sum of the packsts so no need to store the raw packets
        self.sums = packets 
            .iter()
            .map(|p| p.iter().sum::<u32>())
            .collect::<Vec<u32>>();

        self.sums.sort_by(|a, b| b.cmp(a)); //sorting in descending order
    }

    fn part1(&mut self) -> Vec<String> {
        // if there is not at least one sum then reading the file should've failed
        crate::output(self.sums.first().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        vec![format!("{}", self.sums.iter().take(3).sum::<u32>())]
    }
}
