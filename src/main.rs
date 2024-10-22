use AOC_2015::solutions::{Solution, day1};
use std::time::Instant;

fn main() {
    let day = Box::new(day1::Day1::new("(())"));
    solve_day(day);
}

fn solve_day(day: Box<dyn Solution<'_>>) {
    let timer = Instant::now();
    println!("The solution to part 1 is: {}", day.part1());
    println!("Time it took: {}", timer.elapsed().as_secs_f64());

    let timer = Instant::now();
    println!("The solution to part 2 is: {}", day.part2());
    println!("Time it took: {}", timer.elapsed().as_secs_f64());
}