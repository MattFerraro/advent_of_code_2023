use std::fs::read_to_string;

fn get_left_number(line: &str) -> i32 {
    let mut left_number: i32 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            left_number = c.to_digit(10).unwrap() as i32;
            return left_number;
        }
    }
    left_number
}

fn get_right_number(line: &str) -> i32 {
    let mut left_number: i32 = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            left_number = c.to_digit(10).unwrap() as i32;
        }
    }
    left_number
}

fn get_number(line: &str) -> i32 {
    let left_number = get_left_number(line);
    let right_number = get_right_number(line);
    left_number * 10 + right_number
}

fn main() {
    let file = read_to_string("./input.txt").unwrap();

    let mut sum = 0;
    for line in file.lines() {
        // println!("{}", line);
        // println!("{}", get_number(line));
        sum += get_number(line);
    }
    println!("{}", sum);
}
