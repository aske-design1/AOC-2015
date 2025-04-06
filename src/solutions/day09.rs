
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
        let country_list: Vec<_> = self.input.iter().map(|line| {
            let (country1, to_split) = line.split_once(" to ").unwrap();
            let (country2, dist) = to_split.split_once(" = ").unwrap(); 
            (country1, country2, dist.parse::<u32>().unwrap())
        }).collect();

        let cities = country_list.iter().fold(HashMap::with_capacity(1000), |mut cities, (country1, country2, _)| {
            let len = cities.len();
            cities.entry(*country1).or_insert(len);
            let len = cities.len();
            cities.entry(*country2).or_insert(len);
            cities
        });

        country_list.iter().fold(vec![vec![0; cities.len()]; cities.len()], |mut dists, (country1, country2, dist)| {
            if let (Some(&idx1), Some(&idx2)) = (cities.get(country1), cities.get(country2)) {
                dists[idx1][idx2] = *dist;
                dists[idx2][idx1] = *dist;
            }

            dists
        })

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

    fn create_new(&self, mut bitmask: Vec<bool>, idx: usize) -> Box<dyn Path<u32>> {
        bitmask[idx] = true;
        Box::new(
            Self {
                bitmask,
                current_idx: idx,
                total_dist: 0
            }
        )
    }

    fn from_existing(&self, path_from: &Box<dyn Path<u32>>, idx: usize, value: u32) -> Box<dyn Path<u32>> { 
        let mut bitmask = path_from.get_bitmask().clone();
        bitmask[idx] = true; 
        Box::new(Self {
            bitmask,
            current_idx: idx,
            total_dist: value
        })
    }

    fn get_total_dist(&self) -> u32 { self.total_dist }
    fn get_current_idx(&self) -> usize { self.current_idx }
    fn get_bitmask(&self) -> &Vec<bool> { &self.bitmask }
    fn get_bitmask_entry(&self, idx:usize) -> Option<&bool> { self.bitmask.get(idx) }
    fn check_fulfillment_criteria(&self) -> bool {
        for bit in self.bitmask.iter() {
            if !bit { return false }
        }
        true
    }
    fn calculate_dist(&self, grid: &Vec<Vec<u32>>, idx: usize) -> u32 {
        self.total_dist + grid[self.current_idx][idx]
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
