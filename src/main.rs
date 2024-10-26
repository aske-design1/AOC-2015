use std::{
    env, time::Instant
};

use aoc_2015::{
    solutions::Solution,
    input
};


fn main() {
    let args: Vec<String> = env::args().collect();
    match input::parse_args(&args) {
        Ok(day) => print_solution(day),
        Err(e) => panic!("Error: {}", e)
    }
}


fn print_solution(day: Box<dyn Solution<Output = String>>) {
    time_solution("1", day.part1().as_str());
    time_solution("2", day.part2().as_str());
}

fn time_solution(part: &str, solution: &str) {
    let timer = Instant::now();
    match solution {
        "0" | "" => println!("Part {} not implemented", part),
        _ => println!("The solution to part {} is: {}", part, solution),
    }
    println!("Time it took: {}", timer.elapsed().as_secs_f64());
}