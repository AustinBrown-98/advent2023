use std::fs::read_to_string;

fn main() {
    let input = read_lines("./input.txt");
    let mut sum = 0;
    for word in &input {
        let first_number = find_first_number(&word);
        let last_number = find_last_number(&word);
        let combined_num = String::from(first_number.to_string().as_str().to_owned() + last_number.to_string().as_str());
        sum += combined_num.parse::<usize>().unwrap();
    }
    println!("sum {:?}", sum);
}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()
        .lines() 
        .map(String::from)
        .collect()
}


fn find_first_number(word: &str) -> usize {
    println!("word {:?}", word);
    let mut first_number = 0;
    for (_i, c) in word.chars().enumerate() {
        if c.is_digit(10) {
            first_number = c.to_digit(10).unwrap() as usize;
            break;
        }
    }
    first_number
}

fn find_last_number(word: &str) -> usize {
    println!("word {:?}", word);
    let mut last_number = 0;
    for (_i, c) in word.chars().rev().enumerate() {
        if c.is_digit(10) {
            last_number = c.to_digit(10).unwrap() as usize;
            break;
        }
    }
    last_number
}