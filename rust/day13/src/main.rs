use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
enum FoldAction {
    X(usize),
    Y(usize),
}

fn read_input(filename: &String) -> (Vec<(usize, usize)>, Vec<FoldAction>) {
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let parts: Vec<&str> = contents
        .split("\r\n\r\n")
        .collect();

    let input: Vec<(usize, usize)> = parts[0]
        .split("\r\n")
        .map(|l| parse_line(l))
        .collect();

    let actions: Vec<FoldAction> = parts[1]
        .split("\r\n")
        .map(|l| parse_action(l))
        .collect();

    (input, actions)
}

fn parse_line(l: &str) -> (usize, usize) {
    let parts: Vec<&str> = l.split(",").collect();

    let a = parts[0].parse::<usize>().expect("Failed to parse int");
    let b = parts[1].parse::<usize>().expect("Failed to parse int");

    (a, b)
}

fn parse_action(l: &str) -> FoldAction {
    let parts: Vec<&str> = l.trim_start_matches("fold along ")
        .split("=")
        .collect();

    let count = parts[1].parse::<usize>().expect("Failed to parse int");
    match parts[0] {
        "x" => FoldAction::X(count),
        "y" => FoldAction::Y(count),
        _ => panic!("Illegal character: {:?}", parts[0]),
    }
}

fn task1(input: &mut Vec<(usize, usize)>, actions: &Vec<FoldAction>) -> usize {
    match actions[0] {
        FoldAction::X(x) => fold_left(input, x),
        FoldAction::Y(y) => fold_up(input, y),
    }

    count_dots(input)
}

fn fold_left(input: &mut Vec<(usize, usize)>, x: usize) {
    for e in input.iter_mut() {
        if e.0 > x {
            *e = ((x) - (e.0 - x), e.1);
        }
    }

    dedup(input);
}

fn fold_up(input: &mut Vec<(usize, usize)>, y: usize) {
    for e in input.iter_mut() {
        if e.1 > y {
            *e = (e.0, (y) - (e.1 - y));
        }
    }

    dedup(input);
}

#[allow(dead_code)]
fn print_paper(input: &Vec<(usize, usize)>) {
    let (max_x, max_y) = find_max(input);
    let mut paper = vec![vec!['.'; max_x + 1]; max_y + 1];

    for (x, y) in input {
        paper[*y][*x] = '#';
    }

    for line in paper {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }

    println!("");
}

fn find_max(input: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut max = (0, 0);

    for (x, y) in input {
        if x > &max.0 {
            max.0 = *x;
        }

        if y > &max.1 {
            max.1 = *y;
        }
    }

    max
}

fn count_dots(input: &Vec<(usize, usize)>) -> usize {
    input.len()
}

fn dedup(input: &mut Vec<(usize, usize)>) {
    let mut set = HashSet::new();
    let mut remove = vec![];

    for index in 0..input.len() {
        let e = input[index];

        if set.contains(&e) {
            remove.push(index);
            continue;
        }

        set.insert(e);
    }

    for i in 0..remove.len() {
        let index = remove[i];
        input.remove(index - i);
    }
}

fn task2(input: &mut Vec<(usize, usize)>, actions: &Vec<FoldAction>) {
    for action in actions {
        match action {
            FoldAction::X(x) => fold_left(input, *x),
            FoldAction::Y(y) => fold_up(input, *y),
        }
    }

    print_paper(input);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let task_select = &args[2];
    let (mut input, actions) = read_input(filename);

    match task_select.as_str() {
        "1" => {
            let result1 = task1(&mut input, &actions);
            println!("Task 1: {:?}", result1);
        },
        "2" => task2(&mut input, &actions),
        _ => panic!("Invalid task: {:?}", task_select)
    }


}
