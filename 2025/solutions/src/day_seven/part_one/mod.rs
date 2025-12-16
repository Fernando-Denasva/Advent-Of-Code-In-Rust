use core::{error};
use std::io::{BufRead, BufReader};
use std::{fs::File};
use regex::Regex;
use std::collections::hash_set::HashSet;

pub fn solution(input_path: &str) -> Result<u16, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let mut buffer: BufReader<File> = BufReader::new(file);
    
    let mut line_one = String::new();
    buffer.read_line(&mut line_one)?;

    let mut beam_indices_current: HashSet<usize> = HashSet::new();
    beam_indices_current.insert(line_one.find('S').expect("Could not find starting point 'S'!"));

    let splitters: Regex = Regex::new(r"\^").unwrap();
    let mut splitted_times: u16 = 0;
    for line in buffer.lines() {
        let line = line?;
        let splitter_indices: Vec<usize> = splitters.find_iter(&line).map(|i|i.start()).collect();
        let mut beam_indices_temp: HashSet<usize> = HashSet::new();
        for index in beam_indices_current.iter() {
            if splitter_indices.contains(&index)  {
                beam_indices_temp.insert(index+1);
                beam_indices_temp.insert(index-1);
                splitted_times+=1;
            } else {
                beam_indices_temp.insert(*index);
            }
        }

        if splitters.is_match(&line) {
            beam_indices_current = beam_indices_temp;
        }

        for c in line.chars().enumerate() {
            if c.1 == '.' && beam_indices_current.contains(&c.0) {
                print!("|");
            } else {
                print!("{}",c.1);
            }
        }
        println!();
    }
    println!();

    Ok(splitted_times)
}
