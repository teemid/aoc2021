use std::fs;
use std::env;

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct Position {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

fn read_input(filename: &String) -> Vec<Command> {
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    contents
        .split("\r\n")
        .map(|p| parse_command(p))
        .collect()
}

fn parse_command(command: &str) -> Command {
    let parts: Vec<&str> = command.split(" ").collect();
    let number = parts[1].parse().expect("Failed to parse int");
    match parts[0] {
        "forward" => Command::Forward(number),
        "up" => Command::Up(number),
        "down" => Command::Down(number),
        _ => panic!("Unknown command")
    }
}

fn task1(commands: &Vec<Command>) -> i32 {
    let mut position = Position{ horizontal: 0, vertical: 0, aim: 0 };

    for command in commands {
        match command {
            Command::Up(number) => position.vertical -= number,
            Command::Down(number) => position.vertical += number,
            Command::Forward(number) => position.horizontal += number,
        }
    }

    position.horizontal * position.vertical
}

fn task2(commands: &Vec<Command>) -> i32 {
    let mut position = Position{ horizontal: 0, vertical: 0, aim: 0 };

    for command in commands {
        match command {
            Command::Up(number) => position.aim -= number,
            Command::Down(number) => position.aim += number,
            Command::Forward(number) => {
                position.horizontal += number;
                position.vertical += number * position.aim;
            },
        }
    }

    position.horizontal * position.vertical
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = read_input(filename);
    let result1 = task1(&input);
    println!("Task 1: {}", result1);

    let result2 = task2(&input);
    println!("Task 2: {}", result2);
}
