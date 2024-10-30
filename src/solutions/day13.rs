use super::*;
use std::collections::HashMap;

use crate::util::path_finder::{Path, PathFinder};

pub struct Day13 {
    input: Vec<String>,
}

impl Day13 {
    pub fn new(input: String) -> Self {
        let split_by = if input.contains("\r\n") { "\r\n" } else { "\n" };

        Self {
            input: input.split(split_by).map(|line| line.to_string()).collect(),
        }
    }

    fn convert_to_grid(&self, hash: Option<HashMap<&str, usize>>) -> Vec<Vec<i32>> {
        let mut indexer = hash.unwrap_or(HashMap::new());
        
        for line in self.input.iter() {
            Self::add_to_indexer(&mut indexer, line);
        }

        let mut vec = vec![vec![0; indexer.len()]; indexer.len()];
        for line in self.input.iter() {
            Self::parse_line(&mut vec, line, &indexer);
        }
        vec
    }

    fn add_to_indexer<'a>(indexer: &mut HashMap<&'a str, usize>, line: &'a str) {
        let name = line.split(" ").collect::<Vec<&str>>()[0];

        let indexer_len = indexer.len();
        indexer.entry(name).or_insert(indexer_len);
    }

    fn parse_line(add_to: &mut Vec<Vec<i32>>, line: &str, indexer: &HashMap<&str, usize>) {
        let split: Vec<&str> = line.split(" would ").collect();
            let name = split[0];
            let split: Vec<&str> = split[1]
                .split(" happiness units by sitting next to ")
                .collect();
            let points = split[0];
            let by_sitting_next_to = split[1].split(".").collect::<Vec<&str>>()[0];

            let num: i32 = Self::parse_num(points);
            let (&idx1, &idx2) = (
                indexer.get(name).unwrap(),
                indexer.get(by_sitting_next_to).unwrap(),
            );
            add_to[idx1][idx2] = num;
    }
    fn parse_num(points: &str) -> i32 {
        let points: Vec<&str> = points.split(" ").collect();
        let num: i32 = points[1].parse().unwrap();

        match points[0] {
            "lose" => num * -1,
            "gain" => num,
            _ => panic!(),
        }
    }
}

impl Solution for Day13 {
    fn part1(&self) -> String {
        let grid = self.convert_to_grid(None);
        let mut paths = PathFinder::new(Day13Path::new());
        let most_happiness = paths.route(grid, false);        
        format!("{}", most_happiness)
    }
    fn part2(&self) -> String {
        let mut hash = HashMap::new();
        hash.insert("you", 0);

        let grid = self.convert_to_grid(Some(hash));
        let mut paths = PathFinder::new(Day13Path::new());
        let most_happiness = paths.route(grid, false);        
        format!("{}", most_happiness)
    }
}

struct Day13Path {
    bitmask: Vec<bool>,
    total_dist: i32,
    current_idx: usize,
    initial_idx: usize
}

impl Day13Path {
    pub fn new() -> Self {
        Self {
            bitmask: vec![],
            total_dist: 0,
            current_idx: 0,
            initial_idx:0,
        }
    }
}

impl Path<i32> for Day13Path {
    fn create_new(&self, mut bitmask: Vec<bool>, idx: usize) -> Box<dyn Path<i32>> {
        bitmask[idx] = true; 
        Box::new(
            Self {
                bitmask,
                total_dist: 0,
                current_idx: idx,
                initial_idx: idx,
            }
        )
    }
    fn from_existing(&self, path_from: &Box<dyn Path<i32>>, idx: usize, value: i32) -> Box<dyn Path<i32>> {
        let mut bitmask = path_from.get_bitmask().clone();
        bitmask[idx] = true; 

        Box::new(
            Self {
                bitmask,
                total_dist: value,
                current_idx: idx,
                initial_idx: path_from.get_initial(),
            }
        )
    }
    fn get_total_dist(&self) -> i32 { self.total_dist }
    fn get_current_idx(&self) -> usize { self.current_idx }
    fn get_bitmask(&self) -> &Vec<bool> { &self.bitmask }
    fn get_bitmask_entry(&self, idx:usize) -> Option<&bool> { self.bitmask.get(idx) }
    fn get_initial(&self) -> usize { self.initial_idx }
    fn check_fulfillment_criteria(&self) -> bool { !self.bitmask.iter().any(|bit| !(*bit)) }
    fn calculate_dist(&self, grid: &Vec<Vec<i32>>, idx: usize) -> i32 {
        let false_bools = self.bitmask.iter()
        .filter(|bit| !(**bit)).count(); 
        
        self.total_dist + grid[self.current_idx][idx] + grid[idx][self.current_idx] + 
        match false_bools {
            1  => {
                grid[idx][self.initial_idx] + grid[self.initial_idx][idx] 
            },
            _ => 0
        }
    }
    fn print(&self) {
        println!("Path:");
        println!("bitmask: {:?}", self.bitmask);
        println!("dist: {}", self.total_dist);
        println!("current_idx: {}", self.current_idx);
        println!("initial: {}", self.initial_idx);
        println!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = String::from("Alice would gain 54 happiness units by sitting next to Bob.\nAlice would lose 79 happiness units by sitting next to Carol.\nAlice would lose 2 happiness units by sitting next to David.\nBob would gain 83 happiness units by sitting next to Alice.\nBob would lose 7 happiness units by sitting next to Carol.\nBob would lose 63 happiness units by sitting next to David.\nCarol would lose 62 happiness units by sitting next to Alice.\nCarol would gain 60 happiness units by sitting next to Bob.\nCarol would gain 55 happiness units by sitting next to David.\nDavid would gain 46 happiness units by sitting next to Alice.\nDavid would lose 7 happiness units by sitting next to Bob.\nDavid would gain 41 happiness units by sitting next to Carol.");
        let day = Day13::new(input);

        assert_eq!(day.part1(), "330".to_string());
    }
    #[test] 
    fn test2() {
        let input = String::from("Alice would gain 54 happiness units by sitting next to Bob.\r\nAlice would lose 79 happiness units by sitting next to Carol.\r\nAlice would lose 2 happiness units by sitting next to David.\r\nBob would gain 83 happiness units by sitting next to Alice.\r\nBob would lose 7 happiness units by sitting next to Carol.\r\nBob would lose 63 happiness units by sitting next to David.\r\nCarol would lose 62 happiness units by sitting next to Alice.\r\nCarol would gain 60 happiness units by sitting next to Bob.\r\nCarol would gain 55 happiness units by sitting next to David.\r\nDavid would gain 46 happiness units by sitting next to Alice.\r\nDavid would lose 7 happiness units by sitting next to Bob.\r\nDavid would gain 41 happiness units by sitting next to Carol.");
        let day = Day13::new(input);

        assert_eq!(day.part2(), "330".to_string());
    }
}