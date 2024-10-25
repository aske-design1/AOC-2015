
use super::*;
use std::collections::{HashMap, VecDeque};

pub struct Day9 {
    input: Vec<String>,
}


#[derive(Clone)]
struct Path {
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

struct Paths(VecDeque<Path>);
impl Paths {
    fn new() -> Self {
        Paths(VecDeque::new())
    }

    fn add(&mut self, mut bitmask: Vec<bool>, idx: usize, total_dist: u32) {
        bitmask[idx] = true; 
        let path = Path::new(bitmask, total_dist, idx);
        self.0.push_back(path);
    }

    fn pop(&mut self, front: bool) -> Option<Path> {
        match front {
            true => self.0.pop_front(),
            false => self.0.pop_back(),
        }
    }

    fn sort(&mut self) {
        let mut vec: Vec<Path> = self.0.clone().into();
        vec.sort_by(|a, b| {
            let mut dif = a.total_dist.cmp(&(b.total_dist));
            if std::cmp::Ordering::Equal == dif {
                dif = b.bitmask.iter().filter(|bit| **bit).count()
                .cmp(&a.bitmask.iter().filter(|bit| **bit).count());
            }
            dif
        });

        self.0 = VecDeque::from(vec);
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("-------------------- Paths --------------------");
        for path in self.0.iter() {
            path.print();
        }
    }

    fn find_path(&mut self, grid: &Vec<Vec<u32>>, smallest: bool) -> Path {
        while let Some(path) = self.pop(smallest) {
            //checks if bitmask is filled then exit
            if path.is_bitmask_filled() {
                return path
            }

            for (idx, cities_len) in grid[path.current].iter().enumerate() {
                //checking bitmask
                if !path.bitmask[idx] {
                    //adding path with updated bitmask
                    let bitmask = path.bitmask.clone();
                    self.add(bitmask, idx, cities_len + path.total_dist);
                }
            }
            
            //Sort the paths such that the smallest value is first
            self.sort();
        }

        unreachable!()
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

    fn route(&self, cities: Vec<Vec<u32>>) -> u32 {
        let mut paths = Paths::new();
        for (i, _) in cities.iter().enumerate() {
            let bitmask = vec![false; cities.len()];
            paths.add(bitmask, i, 0);
        }
        paths.find_path(&cities, true).total_dist
    }
    
    fn largest_route(&self, cities: Vec<Vec<u32>>) -> u32 {
        let mut paths_to_check = Paths::new();
        for (i, _) in cities.iter().enumerate() {
            let bitmask = vec![false; cities.len()];
            paths_to_check.add(bitmask, i, 0);
        }

        let mut largest_dist: Vec<u32> = Vec::new();
        while let Some(path_to_check) = paths_to_check.pop(false) {
            let mut paths = Paths::new();
            paths.add(path_to_check.bitmask, path_to_check.current, path_to_check.total_dist);
            largest_dist.push(paths.find_path(&cities, false).total_dist);
        }
        largest_dist.sort();
        *largest_dist.last().unwrap()
    }
    

}

impl Solution for Day9 {
    fn part1(&self) -> String {
        let arr_with_cities = self.find_cities();
        let smallest_route = self.route(arr_with_cities);
        format!("{}", smallest_route)
    }
    fn part2(&self) -> String {
        let arr_with_cities = self.find_cities();
        let largest_route = self.largest_route(arr_with_cities);
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
