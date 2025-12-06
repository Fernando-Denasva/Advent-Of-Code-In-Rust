use core::{error};
use std::io::{BufRead, BufReader};
use std::{fs::File};

fn largest(bank: &[u8], largest_joltages: &mut Vec<(usize,u8)>) {
    if largest_joltages.len() == 12 {
        return ();
    }
    let mut largest_battery_at_zero = false;
    let start = if largest_joltages.len() > 0 {
        let latest_index = largest_joltages.last().expect("Could not read last index!").0;
        if latest_index == 0 {
            largest_battery_at_zero = true;
            1
        } else {
            latest_index
        }
    } else {
        0
    };
    let mut current_start = start;
    if start > 0 && largest_battery_at_zero == false {
        current_start+=1;
    }
    let bank_length: usize = bank.len();
    let needed_batteries: usize = 11-largest_joltages.len();
    for number in (1..=9).into_iter().rev() {
        let mut largest_joltage: (usize,u8) = (current_start,bank[current_start]);
        for battery in bank.iter().enumerate() {
            if battery.0 >= current_start {
                if *battery.1 > largest_joltage.1 && *battery.1 <= number {
                    largest_joltage = (battery.0,*battery.1);
                }
            }
        }

        if largest_joltage.0+needed_batteries < bank_length {
            largest_joltages.push(largest_joltage);
            return largest(bank, largest_joltages);
        } else {
            continue;
        }
    }
}

pub fn solution(input_path: &str) -> Result<u64, Box<dyn error::Error>> {
    let file: File = File::open(input_path)?;
    let buffer: BufReader<File> = BufReader::new(file);
    let mut joltages: u64 = 0;
    for line in buffer.lines() {
        let current_bank: String = line.unwrap_or_else(|e|{panic!("Could not read the current line! '{e}'")});
        let current_bank: Vec<u8> = current_bank.chars().map(|b| b.to_digit(10).map(|b| b as u8).unwrap_or_else(||panic!("Cannot convert battery to number!"))).collect::<Vec<u8>>();
        let mut largest_joltages: Vec<(usize,u8)> = Vec::new();

        largest(&current_bank, &mut largest_joltages);
        
        let mut largest_joltage = String::new();
        for battery in largest_joltages {
            largest_joltage.push_str(&battery.1.to_string());
        }
        joltages+=largest_joltage.parse::<u64>()?;

    }
    Ok(joltages)
}