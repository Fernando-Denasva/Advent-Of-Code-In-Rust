use core::error;
use std::io::{BufReader, Read};
use std::{fs::File};
use regex::Regex;

pub fn solution(input_path: &str) -> Result<u64, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let mut buffer: BufReader<File> = BufReader::new(file);
    let mut ranges: String = String::new(); 
    if let Err(e) = buffer.read_to_string(&mut ranges) {
        panic!("Could not read the ranges, '{e}'");
    };
    let remove_commas: Regex = Regex::new("[^,]+").unwrap();
    let mut sum: u64 = 0;
    for r#match in remove_commas.find_iter(&ranges) {
        let current_range: Vec<u64> = r#match.as_str().split('-').map(|m| m.parse::<u64>().unwrap_or_else(|e|panic!("Error Parsing Ranges: '{e}'"))).collect::<Vec<u64>>();
        if current_range.len() != 2 {
            panic!("Error when splitting the ranges!");
        }
        for id in current_range[0]..=current_range[1] {
            let current_id  = id.to_string();
            if current_id.len()%2 != 0 {
                continue;
            }
            let half_one: &str = &current_id[0..current_id.len()/2];
            let half_two: &str = &current_id[(current_id.len()/2)..];
            if half_one == half_two {
                sum += current_id.parse::<u64>()?;
            }
        }
    }

    Ok(sum)
}