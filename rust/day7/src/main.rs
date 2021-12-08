use std::env;
use std::fs;

fn read_input(filename: &String) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn triangle(i: i32) -> i32 {
    let mut sum = 0;

    for x in 0..i + 1 {
        sum += x;
    }

    sum
}

fn task1(input: &Vec<i32>) -> i32 {
    let mut results: Vec<i32> = vec![];

    for alignment in 0..input.len() as i32 {
        let mut sum: i32 = 0;

        for i in 0..input.len() {
            let diff = alignment - input[i];
            sum += diff.abs();
        }

        results.push(sum);
    }

    match results.iter().min() {
        Some(min) => *min,
        None => panic!("Failed to find minimum"),
    }
}

fn task2(input: &Vec<i32>) -> i32 {
    let mut results: Vec<i32> = vec![];

    for alignment in 0..input.len() as i32 {
        let mut sum: i32 = 0;

        for i in 0..input.len() {
            let diff = alignment - input[i];
            sum += triangle(diff.abs());
        }

        results.push(sum);
    }

    match results.iter().min() {
        Some(min) => *min,
        None => panic!("Failed to find minimum"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = read_input(filename);
    let result = task1(&input);
    println!("{:?}", result);

    let result2 = task2(&input);
    println!("{:?}", result2);
}
