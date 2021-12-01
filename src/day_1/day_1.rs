
const INPUT: &str = include_str!("day_1.txt");

fn day_1_1() -> u32 {
    let input = INPUT
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut prev = i32::MAX;

    for num in input {
        if prev < num {
            result += 1;
        }
        prev = num;
    }
    
    result
}


fn day_1_2() -> u32 {
    let input = INPUT
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    let mut prev = i32::MAX;

    for i in 0..input.len() - 2 {
        let current = input[i] + input[i + 1] + input[i + 2];
        if prev < current {
            result += 1;
        }
        prev = current;
    }

    
    result
}

fn main() {
    println!("part 1: {}\npart 2: {}", day_1_1(), day_1_2());
}
