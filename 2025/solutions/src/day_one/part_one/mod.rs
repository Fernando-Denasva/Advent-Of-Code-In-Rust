use core::error;
use std::io::{BufRead, BufReader};
use std::{fs::File};
use regex::Regex;
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn solution(input_path: &str) -> Result<u16, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let buffer: BufReader<File> = BufReader::new(file);
    
    let mut current_position: i32 = 50;
    let mut times_at_zero: u16 = 0;
    for rotation in buffer.lines() {
        let rotation = rotation?;
        let mut clicks: i32 = Regex::new(r"\d+").unwrap().find(&rotation).unwrap_or_else(||{panic!("No rotation found!")}).as_str().parse::<i32>()?;
        if clicks > 99 {
            clicks %= 100;
        }
        if rotation.contains("L") {
            current_position += clicks*-1;
            if current_position < 0 {
                current_position += 99+1 ;
            } else if current_position == 0 {
                times_at_zero += 1;
            }
        } else {
            current_position += clicks;
            if current_position > 99+1 {
                current_position -= 99+1;
            } else if current_position == 99+1 {
                times_at_zero += 1;
            }
        }
    }
    Ok(times_at_zero)
}