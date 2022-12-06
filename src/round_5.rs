use std::io::{BufRead};
use regex::Regex;

fn find_index(find_char: String, chars: &Vec<String>) -> usize {
    chars
        .iter()
        .position(|x| *x == find_char)
        .unwrap()
}

fn get_crates_array(crates: &[String]) -> Vec<Vec<char>> {
    let mut crates_array: Vec<Vec<char>> = vec![];

    let digits = (b'0'..=b'9')     // Start as u8
        .map(|c| c as char)            // Convert all to chars
        .collect::<Vec<char>>();
        
    let columns_indexes = crates.last().unwrap().chars()
                                 .enumerate()
                                 .map(|(i, column)| {
        if digits.iter().any(|&digit| digit == column) {
            return i as i32;
        }

        -1
    }).filter(|&index| index != -1).collect::<Vec<i32>>();

    for index in columns_indexes {
        crates_array.push(crates.iter().map(|row| {
            row.chars().nth(index as usize).unwrap()
        }).collect::<Vec<char>>());
    }

    crates_array
}

fn do_move_9000(amount: &str, from: &str, to: &str, crates_array: &mut Vec<Vec<char>>) {
    let from_index = from.parse::<usize>().unwrap() - 1;
    let to_index = to.parse::<usize>().unwrap() - 1;

    // println!("amount: {:#?}, from: {:#?}, to: {:#?}", amount, from, to);

    for _ in 0..amount.parse::<usize>().unwrap() {
        let element_to_move = crates_array[from_index].iter().position(|crate_element| crate_element.is_alphabetic()).unwrap();
        let mut place_to_move = crates_array[to_index].iter().position(|crate_element| crate_element.is_alphabetic()).unwrap_or(crates_array[to_index].len() - 1);

        // println!("before crates_array: {:#?}", crates_array);

        if place_to_move == 0 {
            crates_array[to_index].insert(0, ' ');
        } else {
            place_to_move = place_to_move - 1;
        }

        crates_array[to_index][place_to_move] = crates_array[from_index][element_to_move];
        crates_array[from_index][element_to_move] = ' ';

        // println!("after crates_array: {:#?}", crates_array);
    }

    ()
}

fn do_move_9001(amount: &str, from: &str, to: &str, crates_array: &mut Vec<Vec<char>>) {
    let from_index = from.parse::<usize>().unwrap() - 1;
    let to_index = to.parse::<usize>().unwrap() - 1;

    let mut crates_to_move: Vec<char> = vec![];

    for _ in 0..amount.parse::<usize>().unwrap() {
        let element_to_move = crates_array[from_index].iter().position(|crate_element| crate_element.is_alphabetic()).unwrap();
        
        crates_to_move.push(crates_array[from_index][element_to_move]);

        crates_array[from_index][element_to_move] = ' ';
    }

    crates_to_move.reverse();

    for crate_to_move in crates_to_move {
        let mut place_to_move = crates_array[to_index].iter().position(|crate_element| crate_element.is_alphabetic()).unwrap_or(crates_array[to_index].len() - 1);

        if place_to_move == 0 {
            crates_array[to_index].insert(0, ' ');
        } else {
            place_to_move = place_to_move - 1;
        }

        crates_array[to_index][place_to_move] = crate_to_move;
    }

    ()
}

fn get_top_crates(crates_array: &Vec<Vec<char>>) -> String {
    let mut result: Vec<char> = vec![];

    for crate_element in crates_array {
        let top_crates: Vec<&char> = crate_element.iter().filter(|crate_element| crate_element.is_alphabetic()).collect();

        if top_crates.len() > 0 {
            result.push(*top_crates[0]);
        }
    }

    String::from_iter(result)
}

pub fn part_1(lines: &Vec<String>) {
    let size: usize = find_index(String::new(), lines);
    let (crates, moves): (&[String], &[String]) = lines.split_at(size);

    let mut crates_array = get_crates_array(crates);

    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    for next_move in moves {
        match re.captures(next_move) {
            Some(movement_data) => do_move_9000(&movement_data[1], &movement_data[2], &movement_data[3], &mut crates_array),
            _ => ()
        }
    }

    println!("part_1: {:#?}", get_top_crates(&crates_array));
}

pub fn part_2(lines: &Vec<String>) {
    let size: usize = find_index(String::new(), lines);
    let (crates, moves): (&[String], &[String]) = lines.split_at(size);

    let mut crates_array = get_crates_array(crates);

    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    for next_move in moves {
        match re.captures(next_move) {
            Some(movement_data) => do_move_9001(&movement_data[1], &movement_data[2], &movement_data[3], &mut crates_array),
            _ => ()
        }
    }

    println!("part_2: {:#?}", get_top_crates(&crates_array));
}

pub fn start() {
    let lines: Vec<String> = std::io::stdin().lock().lines().collect::<Result<_, _>>().unwrap();
    part_1(&lines);
    part_2(&lines);
}