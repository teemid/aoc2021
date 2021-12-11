use std::env;
use std::fs;

fn read_input(filename: &String) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .split("\r\n")
        .map(|l| l.to_string())
        .collect()
}

fn task1(input: &mut Vec<String>) {
    let mut symbols = vec![];
    let mut indices = vec![];
    let mut auto_complete = vec![];

    for i in 0..input.len() {
        let line = &input[i];

        let (c, remainders) = validate_line(&line);
        match c {
            Some(c) => {
                symbols.push(c);
                indices.push(i);
            },
            None => {
                auto_complete.push(remainders);
            },
        }
    }

    let result1 = calculate_syntax_error_score(&symbols);
    let result2 = task2(&auto_complete);

    println!("Task 1: {:?}", result1);
    println!("Task 2: {:?}", result2);
}

fn calculate_syntax_error_score(chars: &Vec<char>) -> usize {
    chars
        .iter()
        .map(|c| synax_error_score_mapping(c))
        .sum()
}

fn synax_error_score_mapping(c: &char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Illegal symbol: {:?}", c),
    }
}

fn validate_line(line: &String) -> (Option<char>, Vec<char>) {
    let mut starters = vec![];

    for c in line.chars() {
        match c {
            '(' => starters.push(c),
            '{' => starters.push(c),
            '[' => starters.push(c),
            '<' => starters.push(c),
            ')' => {
                if starters.last() == Some(&'(') {
                    starters.pop();
                } else {
                    return (Some(c), starters);
                }
            },
            '}' => {
                if starters.last() == Some(&'{') {
                    starters.pop();
                } else {
                    return (Some(c), starters);
                }
            },
            ']' => {
                if starters.last() == Some(&'[') {
                    starters.pop();
                } else {
                    return (Some(c), starters);
                }
            },
            '>' => {
                if starters.last() == Some(&'<') {
                    starters.pop();
                } else {
                    return (Some(c), starters);
                }
            },
            _ => panic!("Invalid input char: {:?}", c),
        }
    }

    return (None, starters);
}


fn task2(input: &Vec<Vec<char>>) -> usize {
    let mut scores = vec![];

    for line in input {
        let completed_line = complete_line(&line);
        let score = calculate_auto_complete_score(&completed_line);
        scores.push(score);
    }

    scores.sort();

    let middle_index = scores.len() / 2;
    let score = scores[middle_index];

    println!("score: {:?}", score);

    score
}

fn complete_line(line: &Vec<char>) -> Vec<char> {
    let mut symbols = vec![];

    for c in line.iter().rev() {
        match c {
            '(' => symbols.push(')'),
            '{' => symbols.push('}'),
            '[' => symbols.push(']'),
            '<' => symbols.push('>'),
            _ => panic!("Illegal symbol: {:?}", c),
        }
    }

    symbols
}

fn calculate_auto_complete_score(completed_line: &Vec<char>) -> usize {
    let mut score = 0;

    for c in completed_line {
        score *= 5;

        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Illegal char: {:?}", c),
        }
    }

    score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut input = read_input(filename);
    task1(&mut input);
}
