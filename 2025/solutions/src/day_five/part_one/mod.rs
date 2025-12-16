use core::{error};
use std::io::{BufReader, Read};
use std::{fs::File};
use regex::{Regex, RegexBuilder};

pub fn solution(input_path: &str) -> Result<u32, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let mut buffer: BufReader<File> = BufReader::new(file);
    let mut map: String = String::new();
    buffer.read_to_string(&mut map)?;

    let ingredient_id_regex: Regex = RegexBuilder::new(r"(?m:^\d+$)").crlf(true).build().expect("Failed to parse ingredient ID regex!");
    let ingredient_id_ranges_regex: Regex = RegexBuilder::new(r"(?m:\d+-\d+)").crlf(true).build().expect("Failed to parse ingredient ID ranges regex!");
    let mut fresh_ingredients: u32 = 0;
    for ingredient_id in ingredient_id_regex.find_iter(&map).map(|id|{id.as_str().parse::<u64>().unwrap_or_else(|e|{panic!("Could not parse ingredient id! {e}")})}) {
        for ingrediet_id_range in ingredient_id_ranges_regex.find_iter(&map) {
            let range = ingrediet_id_range
                                    .as_str()
                                    .split('-')
                                    .map(|id| {id.parse::<u64>().unwrap_or_else(|e|{panic!("Could not parse current range Id! {e}")})})
                                    .collect::<Vec<u64>>();
            if range.len() != 2 {
                panic!("Unexpected range!");
            }
            if ingredient_id >= range[0] && ingredient_id <= range[1] {
                fresh_ingredients+=1;
                break;
            }
        }
    }

    Ok(fresh_ingredients)
}