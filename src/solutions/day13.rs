use super::*;
use crate::util::path_finder::PathFinder;

    pub struct Day13 {
        input: Vec<String>
    }

    impl Day13 {
        pub fn new(input: String) -> Self {
            let split_by = if input.contains("\r\n") { "\r\n" } 
            else { "\n" };

            Self { input: input
                .split(split_by)
                .map(|line| line.to_string())
                .collect() 
            }
        }

        fn convert_to_grid(&self) -> Vec<Vec<i32>> { 
            vec![vec![0]]
        }

    }

    impl Solution for Day13 {
        type Output = String;
        fn part1(&self) -> String { 
            let grid = self.convert_to_grid();
            let most_happiness = PathFinder::<i32>::new().largest_route(grid);
            format!("{}", most_happiness) 
        }
        fn part2(&self) -> String { format!("") } 
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] fn test1() {
            let input = String::from("Alice would gain 54 happiness units by sitting next to Bob.\r\nAlice would lose 79 happiness units by sitting next to Carol.\r\nAlice would lose 2 happiness units by sitting next to David.\r\nBob would gain 83 happiness units by sitting next to Alice.\r\nBob would lose 7 happiness units by sitting next to Carol.\r\nBob would lose 63 happiness units by sitting next to David.\r\nCarol would lose 62 happiness units by sitting next to Alice.\r\nCarol would gain 60 happiness units by sitting next to Bob.\r\nCarol would gain 55 happiness units by sitting next to David.\r\nDavid would gain 46 happiness units by sitting next to Alice.\r\nDavid would lose 7 happiness units by sitting next to Bob.\r\nDavid would gain 41 happiness units by sitting next to Carol.");
            let day = Day13::new(input);

            assert_eq!(day.part1(), "330".to_string());
        }
        /*#[test] fn test2() {
            let input = String::from("Alice would gain 54 happiness units by sitting next to Bob.\r\nAlice would lose 79 happiness units by sitting next to Carol.\r\nAlice would lose 2 happiness units by sitting next to David.\r\nBob would gain 83 happiness units by sitting next to Alice.\r\nBob would lose 7 happiness units by sitting next to Carol.\r\nBob would lose 63 happiness units by sitting next to David.\r\nCarol would lose 62 happiness units by sitting next to Alice.\r\nCarol would gain 60 happiness units by sitting next to Bob.\r\nCarol would gain 55 happiness units by sitting next to David.\r\nDavid would gain 46 happiness units by sitting next to Alice.\r\nDavid would lose 7 happiness units by sitting next to Bob.\r\nDavid would gain 41 happiness units by sitting next to Carol.");
            let day = Day13::new(input);

            assert_eq!(day.part2(), "330".to_string());
        }*/
    }