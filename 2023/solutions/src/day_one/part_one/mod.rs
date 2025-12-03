use core::error;
use std::io::{BufRead, BufReader};
use std::{fs::File};
use regex::Regex;

pub fn solution(input_path: &str) -> Result<u32, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let buffer: BufReader<File> = BufReader::new(file);
    let mut result: u32 = 0;
    for line in buffer.lines() {
        let mut digits_found = String::new();
        let digits = Regex::new(r"\d").unwrap().find_iter(&line?)
        .map(
            |m| m.as_str().parse::<char>().unwrap_or_else(|e|{panic!("{e}")})
        ).collect::<Vec<char>>();
        
        if !digits.is_empty() {
            digits_found.push(*digits.first().unwrap_or_else(||{panic!("Can't find first digit")}));
            digits_found.push(*digits.last().unwrap_or_else(||{panic!("Can't find first digit")}));
        }
        result += digits_found.parse::<u32>()?
    }   
    Ok(result)
}