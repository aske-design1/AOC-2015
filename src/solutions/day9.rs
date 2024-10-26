
use super::*;
use crate::util::path_finder::Paths;
use std::collections::HashMap;

pub struct Day9 {
    input: Vec<String>,
}


pub struct Path {
    bitmask: Vec<bool>,
    total_dist: u32,
    current: usize
}

impl Path {
    pub fn new(bitmask: Vec<bool>, total_dist: u32, current: usize) -> Self {
        Self { bitmask, total_dist, current }
    }
    fn is_bitmask_filled(&self) -> bool {
        for bit in self.bitmask.iter() {
            if !bit { return false }
        }
        true
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Path:\nBitmask: {:?}\nTotal dist: {}\nCurrent: {}", self.bitmask, self.total_dist, self.current)
    }
}

impl Day9 {
    pub fn new(input: String) -> Self {
        Self { input: input.split("\r\n").map(|line| line.to_string()).collect() }
    }
    fn find_cities<'a>(&'a self) -> Vec<Vec<u32>> {   
        let mut cities = HashMap::new();
        for line in self.input.iter() {
            let split_line: Vec<&str> = line.split(" to ").collect();
            let split_line2: Vec<&str> = split_line[1].split(" = ").collect(); 
            let (first_country, sec_country) = (split_line[0], split_line2[0]);

            let len = cities.len();
            cities.entry(first_country).or_insert(len);
            let len = cities.len();
            cities.entry(sec_country).or_insert(len);
        }

        let mut arr: Vec<Vec<u32>> = vec![vec![0; cities.len()]; cities.len()]; 
        for line in self.input.iter() {
            let split_line: Vec<&str> = line.split(" to ").collect();
            let split_line2: Vec<&str> = split_line[1].split(" = ").collect(); 
            let (country1, country2, dist) = 
            (split_line[0], split_line2[0], split_line2[1].parse::<u32>().unwrap());

            let (idx1, idx2) = 
            (*cities.get(&country1).unwrap(), *cities.get(&country2).unwrap());
            arr[idx1][idx2] = dist;
            arr[idx2][idx1] = dist;
        }

        arr
    }
}

impl Solution for Day9 {
    type Output = String;
    fn part1(&self) -> String {
        let arr_with_cities = self.find_cities();
        let smallest_route = Paths::<Path>::new().smallest_route(arr_with_cities);

        format!("{}", smallest_route)
    }
    fn part2(&self) -> String {
        let arr_with_cities = self.find_cities();
        let largest_route = Paths::<u32>::new().largest_route(arr_with_cities);

        format!("{}", largest_route)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test1() {
        let input = "London to Dublin = 464\r\nLondon to Belfast = 518\r\nDublin to Belfast = 141".to_string();
        let day = Day9::new(input);
        assert_eq!(day.part1(), "605".to_string());
    }
    #[test] fn test2() {
        let input = "London to Dublin = 464\r\nLondon to Belfast = 518\r\nDublin to Belfast = 141".to_string();
        let day = Day9::new(input);
        assert_eq!(day.part2(), "982".to_string());
    }
}
