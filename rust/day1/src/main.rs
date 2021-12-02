use std::env;
use std::fs;

fn read_file(filename: &String) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    contents
        .split("\r\n")
        .map(|p| p.parse::<i32>().expect("Failed to parse int"))
        .collect()
}

fn task1(input: &Vec<i32>) -> i32 {
    let mut count = 0;

    let mut prev: i32 = input[0];
    for e in input {
        if e - prev > 0 {
            count += 1;
        }
        prev = *e;
    }

    count
}

fn task2(input: &Vec<i32>) -> i32 {
    let mut converted: Vec<i32> = vec![];
    for i in 0..input.len() - 2 {
        let mut sum = 0;
        for j in 0..3 {
            sum += input[i + j];
        }

        converted.push(sum);
    }

    task1(&converted)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let input = read_file(filename);

    let result1 = task1(&input);
    println!("Task 1: {}", result1);

    let result2 = task2(&input);
    println!("Task 2: {}", result2);
}
