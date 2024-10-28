
use super::*;
use std::collections::HashMap;
use crate::util::path_finder::{Path, PathFinder};

pub struct Day9 {
    input: Vec<String>,
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
    fn part1(&self) -> String {
        let arr_with_cities = self.find_cities();
        let mut paths = PathFinder::new(Day9Path::new());
        let smallest_route = paths.route(arr_with_cities, true);
        format!("{}", smallest_route)
    }
    fn part2(&self) -> String {
        let arr_with_cities = self.find_cities();
        let mut paths = PathFinder::new(Day9Path::new());
        let largest_route = paths.route(arr_with_cities, false);
        format!("{}", largest_route)
    }
}

pub struct Day9Path {
    bitmask: Vec<bool>,
    total_dist: u32,
    current_idx: usize,
}

impl Day9Path {
    pub fn new() -> Self {
        Self{
            bitmask: vec![],
            total_dist: 0,
            current_idx: 0,
        }
    }
}

impl Path<u32> for Day9Path {

    fn create_new(&self, mut bitmask: Vec<bool>, idx: usize, value: u32) -> Box<dyn Path<u32>> {
        bitmask[idx] = true;
        Box::new(
            Self {
                bitmask,
                current_idx: idx,
                total_dist: value
            }
        )
    }

    fn from_existing(&self, bitmask: Vec<bool>, idx: usize, value: u32) -> Box<dyn Path<u32>> { self.create_new(bitmask, idx, value) }

    fn get_total_dist(&self) -> u32 { self.total_dist }
    fn get_current_idx(&self) -> usize { self.current_idx }
    fn get_bitmask(&self) -> &Vec<bool> { &self.bitmask }
    fn get_bitmask_entry(&self, idx:usize) -> Option<bool> {
        match self.bitmask.get(idx) {
            Some(val) => Some(*val),
            None => None
        }
    }
    fn check_fulfillment_criteria(&self) -> bool {
        for bit in self.bitmask.iter() {
            if !bit { return false }
        }
        true
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Path:\nBitmask: {:?}\nTotal dist: {}\nCurrent: {}", self.bitmask, self.total_dist, self.current_idx)
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
