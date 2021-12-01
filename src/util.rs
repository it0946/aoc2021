use std::{io::{BufReader, Read}, fs::File, fmt::Display};

pub trait Day<In, Out: Display> {
    fn parse_input(input: String) -> Vec<In>;
    
    fn part_1(input: &[In]) -> Out;
    fn part_2(input: &[In]) -> Out;
    
    fn run(fp: &'static str) {
        let input = read_input(fp);
        let input = Self::parse_input(input);
    
        println!("part 1: {}\npart 2: {}", Self::part_1(&input), Self::part_2(&input));    
    }
}

/// Just for conveniently reading a file without rewriting this in every solution
pub fn read_input(filepath: &str) -> String {
    let mut buf = String::new();
    
    let f = File::open(filepath)
        .unwrap();

    let _ = BufReader::new(f)
        .read_to_string(&mut buf)
        .unwrap();

    buf
}
