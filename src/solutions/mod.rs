pub mod day1;
pub mod day2; 
pub mod day3; 
pub mod day4;
pub mod day5; 
pub mod day6;
pub mod day7;

pub trait Solution {
    fn part1(&self) -> String; 
    fn part2(&self) -> String;
}

mod template {
    use super::*;

    #[allow(dead_code)]
    pub struct Template {
        input: String
    }

    impl Template {
        pub fn new(input: String) -> Self {
            Self { input }
        }
    }

    impl Solution for Template {
        fn part1(&self) -> String { format!("") }
        fn part2(&self) -> String { format!("") } 
    }

    #[cfg(test)]
    mod tests {
        //use super::*;
        //#[test] fn test1() {}
        //#[test] fn test2() {}
    }
}