use util::*;

struct Day2;

impl Day<String, u32> for Day2 {
    fn part_1(input: &String) -> u32 {
        let (mut depth, mut hor) = (0, 0);

        for line in input.lines() {
            let num = &line[line.len() - 1..].parse().unwrap();

            match &line[..1] {
                "f" => hor += num,
                "d" => depth += num,
                "u" => depth -= num,
                _ => ()
            } 
        }

        depth * hor
    }

    fn part_2(input: &String) -> u32 {
        let [mut depth, mut hor, mut aim] = [0; 3];

        for line in input.lines() {
            let num = &line[line.len() - 1..].parse().unwrap();

            match &line[..1] {
                "f" => {
                    hor += num;
                    depth += aim * num;
                },
                "d" => aim += num,
                "u" => aim -= num,
                _ => ()
            } 
        }

        depth * hor
    }

    fn run(fp: &'static str) {
        let input = read_input(fp);
        println!("part 1: {}\npart 2: {}", Self::part_1(&input), Self::part_2(&input));       
    }
}

fn main() {
    Day2::run("inputs/day_2.txt");
}
