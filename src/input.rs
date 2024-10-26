use crate::solutions::Solution;
use std::{fs, io::Read};
use crate::solutions::{
    day1, 
    day2,
    day3,
    day4,
    day5,
    day6,
    day7,
    day8,
    day9,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    day18,
    day19,
    day20,
    day21,
    day22,
    day23,
    day24,
    day25
};

fn create_day_object(day_num: u8, input: String) -> Result<Box<dyn Solution>, String> {
    match day_num {
        1 => Ok(Box::new(day1::Day1::new(input))),
        2 => Ok(Box::new(day2::Day2::new(&input))),
        3 => Ok(Box::new(day3::Day3::new(input))),
        4 => Ok(Box::new(day4::Day4::new(input))),
        5 => Ok(Box::new(day5::Day5::new(input))),
        6 => Ok(Box::new(day6::Day6::new(input))),
        7 => Ok(Box::new(day7::Day7::new(input))),
        9 => Ok(Box::new(day9::Day9::new(input))),
        10 => Ok(Box::new(day10::Day10::new(input))),
        11 => Ok(Box::new(day11::Day11::new(input))),
        12 => Ok(Box::new(day12::Day12::new(input))),
        13 => Ok(Box::new(day13::Day13::new(input))),
        14 => Ok(Box::new(day14::Day14::new(input))),
        15 => Ok(Box::new(day15::Day15::new(input))),
        16 => Ok(Box::new(day16::Day16::new(input))),
        17 => Ok(Box::new(day17::Day17::new(input))),
        18 => Ok(Box::new(day18::Day18::new(input))),
        19 => Ok(Box::new(day19::Day19::new(input))),
        20 => Ok(Box::new(day20::Day20::new(input))),
        21 => Ok(Box::new(day21::Day21::new(input))),
        22 => Ok(Box::new(day22::Day22::new(input))),
        23 => Ok(Box::new(day23::Day23::new(input))),
        24 => Ok(Box::new(day24::Day24::new(input))),
        25 => Ok(Box::new(day25::Day25::new(input))),
        _ => Err("Not a valid day number".to_string())
    }
}

fn create_day_object_with_bytes(day_num: u8, buffer: Vec<u8>) -> Result<Box<dyn Solution>, String> {
    match day_num {
        8 => Ok(Box::new(day8::Day8::new(buffer))),
        _ => Err("Not a valid day number".to_string())
    }
}


pub fn parse_args(args: &Vec<String>) -> Result<Box<dyn Solution>, String> {
    if args.len() < 3 {
        return Err("Error: Not enough arguments supplied".to_string());
    }
    let (operation, day) = (&args[1], &args[2]);

    let day_num = day.parse::<u8>().expect("Not a digit given");
    let path = format!("./input_files/day{}.txt", day_num); 
    match (operation.as_str(), day_num) {
        ("day", 8) => {
            let mut buffer = Vec::new();
            let mut file = match fs::File::open(&path) {
                Ok(cont) => cont,
                Err(_) => return Err("File not found".to_string()),
            };
            let _ = file.read_to_end(&mut buffer); 
            create_day_object_with_bytes(day_num, buffer)
        },
        ("day", _) => {
            let file_content = match fs::read_to_string(&path) {
                Ok(cont) if cont.len() != 0 => cont,
                Ok(_) => return Err("Contents not inserted into file".to_string()),
                Err(_) => return Err("File not found".to_string()),
            };

            create_day_object(day_num, file_content)
        },
        _ => Err("Invalid Operation given".to_string())
    }
}

