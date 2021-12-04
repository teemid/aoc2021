use std::env;
use std::fs;

fn read_input(filename: &String) -> (Vec<usize>, usize) {
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let strings: Vec<&str> = contents.split("\r\n").collect();

    let length = strings[0].len();
    let mut numbers: Vec<usize> = vec![];
    for string in strings {
        numbers.push(usize::from_str_radix(string, 2).expect("Failed to parse line"));
    }

    (numbers, length)
}

fn task1(lines: &Vec<usize>, bitcount: usize) -> usize {
    let half_length = ((lines.len() as f32) / 2.0 + 0.5) as usize;
    let frequency = calculate_frequency(lines, bitcount);

    let gamma_rate = calculate_gamma_rate(&frequency, half_length, bitcount);
    let epsilon_rate = calculate_epsilon_rate(gamma_rate, bitcount);

    gamma_rate * epsilon_rate
}

fn calculate_frequency(lines: &Vec<usize>, bitcount: usize) -> Vec<usize> {
    let mut frequency: Vec<usize> = vec![0; bitcount];
    for line in lines {
        for i in 0..bitcount {
            let mask = 1 << i;
            let result = line & mask;

            frequency[i] += if result != 0 { 1 } else { 0 };
        }
    }

    frequency
}

fn calculate_gamma_rate(frequency: &Vec<usize>, half_length: usize, bitcount: usize) -> usize {
    let mut gamma_rate = 0;

    for i in 0..bitcount {
        if frequency[i] >= half_length {
            gamma_rate |= 1 << i;
        }
    }

    gamma_rate
}

fn calculate_epsilon_rate(gamma_rate: usize, bitcount: usize) -> usize {
    let invert_mask = (2 << bitcount - 1) - 1;
    !gamma_rate & invert_mask
}

fn task2(lines: &Vec<usize>, bitcount: usize) -> usize {
    let oxygen_generator_rate = filter_report(lines, bitcount, most_frequent_mask);
    let co2_scrubber_rate = filter_report(lines, bitcount, least_frequent_mask);

    oxygen_generator_rate * co2_scrubber_rate
}

fn filter_report(lines: &Vec<usize>, bitcount: usize, f: fn (&Vec<usize>, usize, usize) -> usize) -> usize {
    let mut filtered = copy_lines(lines);
    for index in 0..bitcount {
        let frequency = calculate_frequency(&filtered, bitcount);
        let half_length = ((filtered.len() as f32) / 2.0 + 0.5) as usize;
        let mask = f(&frequency, half_length, bitcount);

        filtered = filter_lines(&filtered, index, bitcount, mask);

        if filtered.len() == 1 {
            break;
        }
    }

    filtered[0]
}

fn copy_lines(lines: &Vec<usize>) -> Vec<usize> {
    let mut copy = vec![];

    for line in lines {
        copy.push(*line);
    }

    copy
}

fn most_frequent_mask(frequency: &Vec<usize>, half_length: usize, bitcount: usize) -> usize {
    let mut masks = 0;
    for i in 0..bitcount {
        if frequency[i] >= half_length {
            masks |= 1 << i;
        }
    }

    masks
}

fn least_frequent_mask(frequency: &Vec<usize>, half_length: usize, bitcount: usize) -> usize {
    let mask = most_frequent_mask(frequency, half_length, bitcount);
    !mask & ((1 << bitcount) - 1)
}

fn filter_lines(lines: &Vec<usize>, index: usize, bitcount: usize, mask: usize) -> Vec<usize> {
    let mut filtered: Vec<usize> = vec![];

    for line in lines {
        let bit_select = 1 << bitcount - 1 - index;
        let bit = line & bit_select;
        let bitmask = mask & bit_select;

        if bit == bitmask {
            filtered.push(*line);
        }
    }

    filtered
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (lines, length) = read_input(&args[1]);
    let result1 = task1(&lines, length);
    let result2 = task2(&lines, length);

    println!("Task 1: {}", result1);
    println!("Task 2: {}", result2);
}
