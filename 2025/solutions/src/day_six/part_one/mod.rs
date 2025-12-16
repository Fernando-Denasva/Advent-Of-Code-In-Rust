use core::{error};
use std::io::{BufReader, Read};
use std::{fs::File};
use regex::{Regex, RegexBuilder};

pub fn solution(input_path: &str) -> Result<u64, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let mut buffer: BufReader<File> = BufReader::new(file);

    let mut problems = String::new();
    buffer.read_to_string(&mut problems)?;

    let symbols: Vec<&str> = RegexBuilder::new(r"(?m:[^\d\s])")
                                .crlf(true)
                                .build()
                                .unwrap()
                                .find_iter(&problems)
                                .map(|n|n.as_str())
                                .collect();
    let mut number_totals: Vec<u64> = symbols.iter().map(|n| {
        if *n == "*" {
            1
        } else {
            0
        }
    }).collect();
    let totals_length = number_totals.len();
    let problem_numbers: Regex = RegexBuilder::new(r"\d+").crlf(true).build().unwrap();

    for n in problem_numbers
                            .find_iter(&problems)
                            .map(|n|{
                                n.as_str().parse::<u64>().unwrap_or_else(|e|panic!("Could not parse number! {e}"))
                            }).enumerate() {
        let position = n.0%totals_length;
        if symbols[position] == "+" {
            number_totals[position] += n.1;
        } else if symbols[position] == "*" {
            number_totals[position] *= n.1;
        }
    }

    Ok(number_totals.iter().sum())
}    