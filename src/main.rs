use AOC_2015::solutions::{Solution, day1};
use std::time::Instant;
use std::fs::read_to_string;

const PATH: &'static str = "./input_files/day1.txt";


fn main() {
    let day = Box::new(day1::Day1::new(handle_file(PATH)));
    solve_day(day);
}

fn solve_day(day: Box<dyn Solution>) {
    let timer = Instant::now();
    println!("The solution to part 1 is: {}", day.part1());
    println!("Time it took: {}", timer.elapsed().as_secs_f64());

    let timer = Instant::now();
    println!("The solution to part 2 is: {}", day.part2());
    println!("Time it took: {}", timer.elapsed().as_secs_f64());
}

fn handle_file(path: &str) -> String {
    match read_to_string(path) {
        Ok(cont) => cont,
        Err(_) => panic!(),
    }
}