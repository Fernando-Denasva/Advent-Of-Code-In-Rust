use core::error;
use std::io::{BufRead, BufReader};
use std::{fs::File};

pub fn solution(input_path: &str) -> Result<u32, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let buffer: BufReader<File> = BufReader::new(file);
    let mut joltages: Vec<u32> = Vec::new();
    for line in buffer.lines() {
        let current_bank = line.unwrap_or_else(|e|{panic!("Could not read the current line! '{e}'")});
        let batteries = current_bank.chars().map(|b| b.to_digit(10).map(|b| b as u8).unwrap_or_else(||panic!("Cannot convert battery to number!"))).collect::<Vec<u8>>();
        let mut largest_battery: u8 = batteries[0];
        let mut second_largest_battery: u8 = batteries[1];
        for battery in batteries.iter().skip(1).enumerate() {
            if battery.0 == current_bank.len()-2 {
                if *battery.1 > second_largest_battery {
                    second_largest_battery = *battery.1;
                }
                break;
            }
            if *battery.1 > largest_battery {
                largest_battery = *battery.1;
                second_largest_battery = batteries[battery.0+1+1];
                continue;
            }
            if *battery.1 > second_largest_battery {
                second_largest_battery = *battery.1;
            }
        }
        let mut joltage = largest_battery.to_string();
        joltage.push_str(&second_largest_battery.to_string());
        joltages.push(joltage.parse::<u32>()?);
    }
    Ok(joltages.iter().sum())
}