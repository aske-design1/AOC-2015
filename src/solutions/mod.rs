pub mod day1;
pub mod day2; 
pub mod day3; 
pub mod day4;
pub mod day5; 
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10; 
pub mod day11;
pub mod day12; 
pub mod day13;
pub mod day14; 
pub mod day15;
pub mod day16; 
pub mod day17;
pub mod day18; 
pub mod day19;
pub mod day20; 
pub mod day21;
pub mod day22; 
pub mod day23;
pub mod day24; 
pub mod day25;

pub trait Solution {
    type Output;
    fn part1(&self) -> Self::Output; 
    fn part2(&self) -> Self::Output;
}

mod template {
    use super::*;

    #[allow(dead_code)]
    pub struct Template {
        input: String
    }

    impl Template {
        #[allow(dead_code)]
        pub fn new(input: String) -> Self {
            Self { input }
        }
    }

    impl Solution for Template {
        type Output = String;
        fn part1(&self) -> String { 0.to_string() }
        fn part2(&self) -> String { 0.to_string() } 
    }

    #[cfg(test)]
    mod tests {
        //use super::*;
        //#[test] fn test1() {}
        //#[test] fn test2() {}
    }
}