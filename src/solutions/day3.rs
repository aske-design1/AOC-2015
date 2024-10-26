use super::Solution;
use std::collections::HashSet;

pub struct Day3 {
    input: String
}

impl Day3 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Solution for Day3 {
    type Output = String;

    fn part1(&self) -> String {
        let mut cur_coordinate = (0,0);
        let mut houses: HashSet<(i32, i32)> = HashSet::new();
        
        houses.insert(cur_coordinate);
        for dir in self.input.chars() {
            match dir {
                '<' => cur_coordinate.0 -= 1,
                '>' => cur_coordinate.0 += 1,
                '^' => cur_coordinate.1 += 1,
                'v' => cur_coordinate.1 -= 1,
                _ => panic!()
            };

            houses.insert(cur_coordinate);
        }
        format!("{}", houses.len())
    }
    fn part2(&self) -> String {
        let mut cur_coordinate = (0,0);
        let mut robo_santa = (0,0);
        let mut houses: HashSet<(i32, i32)> = HashSet::new();
        
        houses.insert(cur_coordinate);
        for (idx, dir) in self.input.chars().enumerate() {
            let person = 
            if idx % 2 == 0 { &mut cur_coordinate } 
            else { &mut robo_santa };

            match dir {
                '<' => (*person).0 -= 1,
                '>' => (*person).0 += 1,
                '^' => (*person).1 += 1,
                'v' => (*person).1 -= 1,
                _ => panic!()
            };  

            houses.insert(*person);
        }
        format!("{}", houses.len())
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = Day3::new("^v^v^v^v^v".to_string());
        assert_eq!(test.part1().parse::<u32>().unwrap(), 2);
    }
    #[test]
    fn test2() {
        let test = Day3::new("^v^v^v^v^v".to_string());
        assert_eq!(test.part2().parse::<u32>().unwrap(), 11);
    }
}