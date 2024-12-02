use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();

    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();

    // read through the lines and add the numbers to the lists
    for line in lines{
        let parts = line.split("   ").collect::<Vec<&str>>();
        left_side.push(parts[0].parse::<i32>().unwrap());
        right_side.push(parts[1].parse::<i32>().unwrap());
    }

    // sort
    left_side.sort();
    right_side.sort();

    // part 1
    let mut part1_sum = 0;
    let mut count = 0;
    for left in &left_side{
        part1_sum += (*left - right_side[count]).abs();
        count += 1;
    }
    println!("Part1 Sum: {}", part1_sum);

    // part 2
    let mut part2_sum = 0;
    for left in &left_side{
        let mut appearances = 0;
        for right in &right_side{
            if *left == *right{
                appearances += 1;
            }
        }
        part2_sum += *left * appearances;
    }
    println!("Part2 Sum: {}", part2_sum);
}
