use std::{ops::RangeInclusive};

pub struct Aoc2022_04 {
    pairs: Vec<Vec<u128>>,
}

impl Aoc2022_04 {
    pub fn new() -> Self {
        Self { pairs: vec![] }
    }

    fn convert_range_to_num(range: &RangeInclusive<usize>) -> u128 {
        let mut arr = [0u8; 128];

        for idx in 0..arr.len() {
            if range.contains(&idx) {
                arr[idx] = 1;
            }
        }

        //this unwrap cannot fail, arr contains only zeroes and ones as per above
        aoc::bits_to_num(&arr).unwrap()
    }
}

impl crate::Runner for Aoc2022_04 {
    fn name(&self) -> (usize, usize) {
        (2022, 04)
    }

    fn parse(&mut self) -> () {
        let file = "input/2022-04.txt";

        let content: Vec<String> = aoc::read_linedata_from_file(file);

        let mut ranges = content
            .iter()
            .map(|line| {
                line.split(",")
                    .map(|r| {
                        // unwrap is fine if file is parsed properly
                        let (upper, lower) = r.split_once("-").unwrap();
                        lower.parse::<usize>().unwrap()..=upper.parse::<usize>().unwrap()
                    })
                    .collect::<Vec<RangeInclusive<usize>>>()
            })
            .collect::<Vec<Vec<RangeInclusive<usize>>>>();

        //sorting the ranges so the first is always the smallest
        for v in ranges.iter_mut() {
            v.sort_by(|a, b| a.clone().count().cmp(&b.clone().count()))
        }

        self.pairs = ranges
            .iter()
            .map(|v| {
                v.iter()
                    .map(|range| Self::convert_range_to_num(range))
                    .collect()
            })
            .collect::<Vec<Vec<u128>>>();
    }

    fn part1(&mut self) -> Vec<String> {

        let mut res = 0;

        for pair in self.pairs.iter() {
            if pair[0] == pair[0] & pair[1] {
                res += 1;
            }
        }

        crate::output(res)
    }

    fn part2(&mut self) -> Vec<String> {
        
        let mut res = 0;

        for pair in self.pairs.iter() {
            if pair[0] & pair[1] != 0u128 {
                res += 1;
            }
        }

        crate::output(res)
    }
}
