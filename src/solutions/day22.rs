use crate::rpg_simulator::entities::Boss;

use super::*;

    pub struct Day22 {
        boss: Boss
    }

    impl Day22 {
        pub fn new(input: String) -> Self {
            Self { boss: Boss::new(&input) }
        }
    }

    impl Solution for Day22 {
        fn part1(&self) -> String { 
            

            0.to_string()
        }
        fn part2(&self) -> String { format!("") } 
    }

    #[cfg(test)]
    mod tests {
        //use super::*;
        //#[test] fn test1() {}
        //#[test] fn test2() {}
    }