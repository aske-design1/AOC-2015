use super::*;

use crate::rpg_simulator::entities::{sword::SwordUser, wizard::Wizard};

    #[allow(dead_code)]
    pub struct Day22 {
        boss: SwordUser
    }

    impl Day22 {
        pub fn new(input: String) -> Self {
            Self { boss: SwordUser::new_from_input(&input) }
        }


        fn battle(you: Wizard, boss: SwordUser) -> u32 {
            let mut least_mana = u32::MAX; 

            //Queue and insert


            //Main loop that gathers a prev recorded wizard


            least_mana
        }
    }

    impl Solution for Day22 {
        fn part1(&self) -> String { 
            Self::battle(Wizard::new(50, 500), self.boss.clone()).to_string()
        }
        fn part2(&self) -> String { format!("") } 
    }

    #[cfg(test)]
    mod tests {
        //use super::*;
        //#[test] fn test1() {}
        //#[test] fn test2() {}
    }