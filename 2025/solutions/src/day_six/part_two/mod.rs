use core::{error};
use std::io::{ BufReader, Read};
use std::{fs::File};
use regex::{Regex, RegexBuilder};

pub fn solution(input_path: &str) -> Result<u64, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let mut buffer: BufReader<File> = BufReader::new(file);

    let mut problems = String::new();
    buffer.read_to_string(&mut problems)?;
    problems = problems.replace("\r\n", " \r\n"); // Add one trailing space so all columns have the same format

    let operations: Vec<&str> = RegexBuilder::new(r"(?m:\*|\+)\s+").crlf(true).build().unwrap().find_iter(&problems).map(|n|n.as_str()).collect();
    let mut operations_plus_digit_count: Vec<(usize,&str)> = Vec::new();
    for operation in operations {
        operations_plus_digit_count.push((operation.len()-1,operation.trim()));
    }
    if let Some(o) = operations_plus_digit_count.last_mut() {
        *o = (o.0+1,o.1)
    } else {
        panic!("Could not retrieve last symbol!");
    }

    let filter: Regex = Regex::new(r"[^\n\r]").unwrap();
    let mut operations_cursor: usize = 0;
    let mut target_digits_count = operations_plus_digit_count[operations_cursor].0;
    let mut current_count: usize = 0;
    let mut raw_number = String::new();
    let mut raw_numbers: Vec<String> = Vec::new();
    for char in problems.chars().filter(|c| filter.is_match(&c.to_string()) ).enumerate() {
        if char.1 == '*' || char.1 == '+' {
            break;
        }
        if target_digits_count == current_count {
            raw_numbers.push(raw_number.clone());
            operations_cursor+=1;
            if operations_cursor == operations_plus_digit_count.len() {
                operations_cursor=0;
            }
            target_digits_count = operations_plus_digit_count[operations_cursor].0;
            current_count=0;
            raw_number="".to_string();
        } else {
            if char.1 == ' ' {
                raw_number.push('0');
            } else {
                raw_number.push(char.1);
            }
            current_count+=1;
        }
    }
    let mut grand_totals: Vec<u64> = Vec::new();
    for n in 0..operations_plus_digit_count.len() {
        let mut pivot: Vec<String> = Vec::new();
        let columns: Vec<String> = raw_numbers.iter().skip(n).enumerate().filter(|num| num.0%operations_plus_digit_count.len() == 0).map(|num| num.1.to_string()).collect();
        for i in 0..columns[0].len() {
            let mut pivot_number = String::new();
            for column in columns.iter() {
                pivot_number.push_str(&column[i..i+1].replace("0", ""));
            }
            pivot.push(pivot_number);
        }
        let mut total: u64 = if operations_plus_digit_count[n].1 == "+" {
            0
        } else {
            1
        };
        for parsed_number in pivot.iter().map(|n|n.parse::<u64>().unwrap_or_else(|e|panic!("Could not parse pivoted number! '{e}'"))) {
            if operations_plus_digit_count[n].1 == "+" {
                total+=parsed_number;
            } else {
                total*=parsed_number;
            }
        }
        grand_totals.push(total);
    }

    Ok(grand_totals.iter().sum())
}    