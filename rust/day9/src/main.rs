use std::collections::HashSet;
use std::env;
use std::fs;

fn read_input(filename: &String) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .split("\r\n")
        .map(|l| parse_line(l))
        .collect()
}

fn parse_line(l: &str) -> Vec<u32> {
    let mut line = vec![];

    for c in l.chars() {
        line.push(c.to_digit(10).expect("Failed to parse digit"));
    }

    line
}

fn task1(input: &Vec<Vec<u32>>) -> u32 {
    let low_points = find_low_points(input);

    low_points
        .iter()
        .map(|p| input[p.0][p.1])
        .fold(0, |acc, l| acc + l + 1)
}

fn find_low_points(input: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points = vec![];

    for i in 0..input.len() {
        let row = &input[i];
        for j in 0..row.len() {
            let n = sample_neighbour_n(input, i, j);
            let s = sample_neighbour_s(input, i, j);
            let w = sample_neighbour_w(input, i, j);
            let e = sample_neighbour_e(input, i, j);
            let v = input[i][j];

            if v < n && v < s && v < w && v < e {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn sample_neighbour_n(input: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    sample_neighbour(input, i as isize - 1, j as isize)
}

fn sample_neighbour_s(input: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    sample_neighbour(input, i as isize + 1, j as isize)
}

fn sample_neighbour_w(input: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    sample_neighbour(input, i as isize, j as isize - 1)
}

fn sample_neighbour_e(input: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    sample_neighbour(input, i as isize, j as isize + 1)
}

fn sample_neighbour(input: &Vec<Vec<u32>>, i: isize, j: isize) -> u32 {
    let length = input.len() as isize;
    if i < 0 || i > length - 1 {
        return 9;
    }

    let row = &input[0];
    let row_length = row.len() as isize;
    if j < 0 || j > row_length - 1 {
        return 9;
    }

    input[i as usize][j as usize]
}

fn task2(input: &Vec<Vec<u32>>) -> u32 {
    let low_points = find_low_points(input);

    let mut basins = vec![];
    for low_point in low_points {
        let basin = find_basin(input, low_point);
        basins.push(basin);
    }

    basins.sort_by(|a, b| b.cmp(a));

    assert!(basins.len() > 3, "Number of basins is too small");

    basins[0] * basins[1] * basins[2]
}

fn find_basin(input: &Vec<Vec<u32>>, low_point: (usize, usize)) -> u32 {
    let mut basin = vec![];
    let mut search = vec![low_point];
    let mut visited = HashSet::new();

    while search.len() > 0 {
        let p = search.pop().unwrap();
        let (i, j) = p;

        let n = sample_neighbour_n(input, i, j);
        let s = sample_neighbour_s(input, i, j);
        let w = sample_neighbour_w(input, i, j);
        let e = sample_neighbour_e(input, i, j);

        if i > 0 {
            let p1 = (i - 1, j);
            if add_point_to_search(&visited, p1, n) {
                search.push(p1);
            }
        }

        let p2 = (i + 1, j);
        if add_point_to_search(&visited, p2, s) {
            search.push(p2);
        }

        if j > 0 {
            let p3 = (i, j - 1);
            if add_point_to_search(&visited, p3, w) {
                search.push(p3);
            }
        }

        let p4 = (i, j + 1);
        if add_point_to_search(&visited, p4, e) {
            search.push(p4);
        }

        if !visited.contains(&p) {
            visited.insert(p);
            basin.push(p);
        }
    }

    basin.len() as u32
}

fn add_point_to_search(visited: &HashSet<(usize, usize)>, p: (usize, usize), value: u32) -> bool {
    value < 9 && !visited.contains(&p)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let task_select = &args[2];
    let input = read_input(filename);

    match task_select.as_str() {
        "1" => {
            let result1 = task1(&input);
            println!("Task 1: {:?}", result1);
        }
        "2" => {
            let result = task2(&input);
            println!("Task 2: {:?}", result);
        }
        _ => println!("Unknown task"),
    }
}
