use crate::solutions::Solution;
use std::fs;

pub fn parse_args(args: &Vec<String>) -> Result<Box<dyn Solution>, String> {
    if args.len() < 3 {
        return Err("Error: Not enough arguments supplied".to_string());
    }
    let (operation, day) = (&args[1], &args[2]);
    match operation.as_str() {
        "day" => {
            let day_num = day.parse::<u8>().expect("Not a digit given");
            let path = format!("./input_files/day{}.txt", day_num); 


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


use crate::solutions::{
    day1, 
    day2,
    day3,
    day4,
    day5,
    day6,
    day7
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
        _ => Err("Not a valid day number".to_string())
    }
}