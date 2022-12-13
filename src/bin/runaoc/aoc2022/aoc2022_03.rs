pub struct Aoc2022_03 {
    rsacks: Vec<String>,
}

impl Aoc2022_03 {
    pub fn new() -> Self {
        Self { rsacks: vec![] }
    }

    fn calc_proiority(ch: &char) -> u32 {
        if ch.is_uppercase() {
            u32::from(*ch) - u32::from('A') + 27
        } else {
            u32::from(*ch) - u32::from('a') + 1
        }
    }
}

impl crate::Runner for Aoc2022_03 {
    fn name(&self) -> (usize, usize) {
        (2022, 03)
    }

    fn parse(&mut self) -> () {
        let file = "input/2022-03.txt";

        self.rsacks = aoc::read_linedata_from_file(file);
    }

    fn part1(&mut self) -> Vec<String> {
        let split_sacks = self
            .rsacks
            .iter()
            .map(|s| {
                vec![
                    s.chars().take(s.len() / 2).collect::<String>(),
                    s.chars().rev().take(s.len() / 2).collect::<String>(),
                ]
            })
            .collect::<Vec<Vec<String>>>();

        let mut res = vec![];

        for compartment in split_sacks.iter() {
            for ch in compartment[0].chars() {
                if compartment[1].contains(ch) {
                    res.push(ch);
                    break;
                }
            }
        }

        let ans = res
            .iter()
            .map(|ch| Self::calc_proiority(ch))
            .sum::<u32>();

        crate::output(ans)
    }

    fn part2(&mut self) -> Vec<String> {
        let groups = self.rsacks.chunks(3);

        let mut res = vec![];

        for group in groups {
            for ch in group[0].chars() {
                if group[1].contains(ch) && group[2].contains(ch) {
                    res.push(ch);
                    break;
                }
            }
        }

        let ans = res
            .iter()
            .map(|ch| Self::calc_proiority(ch))
            .sum::<u32>();

        crate::output(ans)
    }
}
