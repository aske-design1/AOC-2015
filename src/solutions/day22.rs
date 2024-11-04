use super::*;

use crate::rpg_simulator::entities::{Boss, Wizard};

    #[allow(dead_code)]
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
            Wizard::new(100, 500).win_with_least_mana(&self.boss).to_string()
        }
        fn part2(&self) -> String { format!("") } 
    }

    #[cfg(test)]
    mod tests {
        //use super::*;
        //#[test] fn test1() {}
        //#[test] fn test2() {}
    }