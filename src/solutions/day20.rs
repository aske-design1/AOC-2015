use super::*;

    #[allow(dead_code)]
    pub struct Day20 {
        presents: u64
    }

    impl Day20 {
        pub fn new(input: String) -> Self {
            Self { presents: input.parse().unwrap() }
        }

        fn find_lowest_house_num(&self, calculate_present_func: fn(house_num: u64) -> u64) -> u64 {
            let mut house = 1;
            while calculate_present_func(house) < self.presents { house*=2 }
            house /= 3;
            while calculate_present_func(house) < self.presents { house+=1 }
            house
        }
        
        fn calculate_presents(house_num: u64) -> u64 {
            let mut presents = 0; 
            let to = (house_num as f64).sqrt() as u64;

            for house in 1..=to {
                if house_num % house == 0 {
                    presents += house;

                    let corresponding_divisor = house_num / house;
                    if corresponding_divisor != house {
                        presents += corresponding_divisor;
                    }
                }
            }
            presents * 10
        }

        fn calculate_presents_with_lazy_elves(house_num: u64) -> u64 {
            let mut presents = 0; 
            let to = (house_num as f64).sqrt() as u64;

            for house in 1..=to {
                if house_num % house == 0 {
                    if house_num / house <= 50 {
                        presents += house;
                    }

                    let corresponding_divisor = house_num / house;
                    if corresponding_divisor != house 
                    && house_num / corresponding_divisor <= 50 {
                        presents += corresponding_divisor;
                    }
                }
            }
            presents * 11
        }
    }

    impl Solution for Day20 {
        fn part1(&self) -> String { 
            self.find_lowest_house_num(Self::calculate_presents).to_string()
        }
        fn part2(&self) -> String { 
            self
            .find_lowest_house_num(
                Self::calculate_presents_with_lazy_elves
            ).to_string() 
        } 
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] 
        fn part_one_test_one() {
            let input = "150";
            let day = Day20::new(input.to_string());

            assert_eq!(day.part1(), 8.to_string());

        }
        #[test] fn part_one_test_two() {
            let input = "120";
            let day = Day20::new(input.to_string());

            assert_eq!(day.part1(), 6.to_string());
        }
        #[test] 
        fn compute_presents_func() {
            assert_eq!(10, Day20::calculate_presents(1));
            assert_eq!(30, Day20::calculate_presents(2));
            assert_eq!(40, Day20::calculate_presents(3));
            assert_eq!(70, Day20::calculate_presents(4));
            assert_eq!(60, Day20::calculate_presents(5));
            assert_eq!(120, Day20::calculate_presents(6));
            assert_eq!(80, Day20::calculate_presents(7));
            assert_eq!(150, Day20::calculate_presents(8));
            assert_eq!(130, Day20::calculate_presents(9));
        }
    }