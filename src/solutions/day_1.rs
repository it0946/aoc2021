#![feature(array_windows)]

use util::*;

struct Day1;

impl Day<Vec<u32>, usize> for Day1 {
    fn parse_input(input: String) -> Option<Vec<u32>> {
        let out = input.lines().map(|s| s.parse().unwrap()).collect();
        Some(out)
    }

    fn part_1(parsed: &Vec<u32>) -> usize {
        parsed.array_windows::<2>()
            .filter(|[a, b]| a < b)
            .count()
    }

    fn part_2(parsed: &Vec<u32>) -> usize {
        let mut prev = u32::MAX;
        parsed.array_windows::<3>()
            .filter(|[a, b, c]| {
                let res = prev < a + b + c;
                prev = a + b + c;
                res
            })
            .count()
    }
}

fn main() {
    Day1::run("inputs/day_1.txt");
}
