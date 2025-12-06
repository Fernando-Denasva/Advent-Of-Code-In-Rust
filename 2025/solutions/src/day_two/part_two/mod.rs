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

            for chunk_size in 0..current_id.len()/2 {
                if current_id.len()%(chunk_size+1) == 0 {
                    let chunks = current_id
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(chunk_size+1).map(
                        |c| c.iter().collect::<String>()
                    )
                    .collect::<Vec<String>>();

                    let mut all_equal = true;
                    let mut result = String::new();
                    for c in chunks.iter() {
                        if c != &chunks[0] {
                            all_equal = false;
                            break;
                        }
                        result.push_str(c);
                    }
                    if all_equal {
                        sum += result.parse::<u64>()?;
                        break;
                    }
                }
            }
        }
    }

    Ok(sum)
}