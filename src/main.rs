use std::{
    env, time::Instant
};

use AOC_2015::{
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

fn print_solution(day: Box<dyn Solution>) {
    let timer = Instant::now();
    println!("The solution to part 1 is: {}", day.part1());
    println!("Time it took: {}", timer.elapsed().as_secs_f64());

    let timer = Instant::now();
    println!("The solution to part 2 is: {}", day.part2());
    println!("Time it took: {}", timer.elapsed().as_secs_f64());
}

