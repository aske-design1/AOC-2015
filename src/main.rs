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

fn print_solution(day: Box<dyn Solution>) {
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

/*' Solution Trait must implement 'send' for it to be able to compile
fn print_solution(day: Box<dyn Solution>) {
    // Part 1 in a separate thread
    let timer1 = Instant::now();
    let part1_thread = thread::spawn(move || {
        let result = day.part1(); // Call part1
        (result, timer1.elapsed()) // Return result and elapsed time
    });

    // Part 2 in a separate thread
    let timer2 = Instant::now();
    let part2_thread = thread::spawn(move || {
        let result = day.part2(); // Call part2
        (result, timer2.elapsed()) // Return result and elapsed time
    });

    // Wait for both threads to finish and collect results
    let (part1_result, part1_time) = part1_thread.join().expect("Part 1 thread panicked");
    let (part2_result, part2_time) = part2_thread.join().expect("Part 2 thread panicked");

    // Print results
    println!("The solution to part 1 is: {}", part1_result);
    println!("Time it took for part 1: {}", part1_time.as_secs_f64());

    println!("The solution to part 2 is: {}", part2_result);
    println!("Time it took for part 2: {}", part2_time.as_secs_f64());
}

*/