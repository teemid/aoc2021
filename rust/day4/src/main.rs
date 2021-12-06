use std::env;
use std::fs;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<usize>>,
    marker_board: Vec<Vec<usize>>,
    has_bingo: bool,
}

impl Board {
    fn new(board: Vec<Vec<usize>>) -> Self {
        Board {
            board: board,
            marker_board: vec![vec![0; 5]; 5],
            has_bingo: false,
        }
    }

    fn mark(&mut self, i: usize, j: usize) {
        self.marker_board[i][j] = 1;
    }

    fn find_value(&self, number: usize) -> Option<(usize, usize)> {
        for i in 0..self.board.len() {
            let line = &self.board[i];
            for j in 0..line.len() {
                if self.board[i][j] == number {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn is_row_bingo(&self, index: usize) -> bool {
        let line = &self.marker_board[index];

        for e in line {
            if *e == 0 {
                return false;
            }
        }

        true
    }

    fn is_col_bingo(&self, index: usize) -> bool {
        for line in &self.marker_board {
            if line[index] == 0 {
                return false;
            }
        }

        true
    }

    fn sum_of_unmarked(&self) -> usize {
        let mut sum = 0;

        for i in 0..self.board.len() {
            for j in 0..self.board.len() {
                if self.marker_board[i][j] == 0 {
                    sum += self.board[i][j];
                }
            }
        }

        sum
    }
}

fn read_input(filename: &String) -> (Vec<usize>, Vec<Board>) {
    let contents = fs::read_to_string(filename).expect("Failed to read input file.");

    let mut strings: Vec<&str> = contents.split("\r\n\r\n").collect();

    let numbers: Vec<usize> = strings[0]
        .split(",")
        .map(|n| n.parse::<usize>().expect("Failed to parse number"))
        .collect();

    let boards: Vec<Board> = strings
        .drain(1..strings.len())
        .map(|b| parse_board(b))
        .collect();

    (numbers, boards)
}

fn parse_board(string: &str) -> Board {
    let board: Vec<Vec<usize>> = string.split("\r\n").map(|l| parse_line(l)).collect();

    Board::new(board)
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split(" ")
        .filter(|&s| !s.is_empty())
        .map(|n| n.parse::<usize>().expect("Failed to parse number"))
        .collect()
}

fn task1(numbers: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    for number in numbers {
        for board in &mut *boards {
            match board.find_value(*number) {
                Some((i, j)) => {
                    board.mark(i, j);

                    let row_bingo = board.is_row_bingo(i);
                    let col_bingo = board.is_col_bingo(j);
                    if row_bingo || col_bingo {
                        let sum = board.sum_of_unmarked();

                        println!("{:?} {:?}", number, sum);

                        return sum * number;
                    }
                }
                None => (),
            }
        }
    }

    0
}

fn task2(numbers: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    let mut bingo_count = 0;

    for number in numbers {
        for board_index in 0..boards.len() {
            let board_length = boards.len();
            let board = &mut boards[board_index];

            if board.has_bingo {
                continue;
            }

            match board.find_value(*number) {
                Some((i, j)) => {
                    board.mark(i, j);

                    let row_bingo = board.is_row_bingo(i);
                    let col_bingo = board.is_col_bingo(j);
                    if row_bingo || col_bingo {
                        if bingo_count == board_length - 1 {
                            let sum = board.sum_of_unmarked();

                            return sum * number;
                        } else {
                            board.has_bingo = true;
                            bingo_count += 1;
                        }
                    }
                }
                None => (),
            }
        }
    }

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let task_select = &args[2];

    let (numbers, mut boards) = read_input(filename);

    match task_select.as_str() {
        "1" => {
            let result1 = task1(&numbers, &mut boards);
            println!("Task 1: {:?}", result1);
        },
        "2" => {
            let result2 = task2(&numbers, &mut boards);
            println!("Task 2: {:?}", result2);
        },
        _ => ()
    }
}
