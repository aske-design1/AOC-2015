use super::*;

    #[allow(dead_code)]
    pub struct Day17 {
        input: Vec<u32>
    }

    impl Day17 {
        pub fn new(input: String) -> Self {
            let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
            let input = input.split(splitter)
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

            Self { input }
        }

        fn compute_combinations(&self, goal: u32) -> u32 {
            let mut accepted: Vec<u32> = Vec::new();
            Self::compute_combinations_helper(&self.input, goal, 0, 0, 0, &mut accepted);
            accepted.len() as u32
        }

        fn compute_combinations_minimum(&self, goal: u32) -> u32 {
            let mut accepted: Vec<u32> = Vec::new();
            Self::compute_combinations_helper(&self.input, goal, 0, 0, 0, &mut accepted);
            accepted.sort();
            accepted.iter().filter(|el| accepted[0] == **el ).count() as u32
        }

        fn compute_combinations_helper(
            containers: &Vec<u32>, 
            goal: u32, idx: usize, 
            cumulative_liters: u32, 
            len: u32,
            accepted: &mut Vec<u32>,
        ) {
            if cumulative_liters == goal {
                accepted.push(len)
            } else if cumulative_liters > goal || idx >= containers.len() {
                return 
            } else {
                Self::compute_combinations_helper(containers, goal, idx+1, cumulative_liters, len, accepted);
                Self::compute_combinations_helper(
                    containers, goal, idx+1, cumulative_liters + containers[idx], len + 1, accepted
                );
            }
        }

    }

    impl Solution for Day17 {
        fn part1(&self) -> String {
            //For test and input
            let goal = 150u32;

            self.compute_combinations(goal).to_string()
        }
        fn part2(&self) -> String { 
            let goal = 150u32;
            self.compute_combinations_minimum(goal).to_string()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] fn test1() {
            let input = "20\n15\n10\n5\n5";
            let day = Day17::new(input.to_string());
            assert_eq!(day.part1(), 4.to_string())
        }
        #[test] 
        fn test2() {
            let input = "20\n15\n10\n5\n5";
            let day = Day17::new(input.to_string());
            assert_eq!(day.part2(), 3.to_string())
        }
    }