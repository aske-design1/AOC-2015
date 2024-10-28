use super::*;
use std::{
    cmp::Ordering,
    collections::{binary_heap, BinaryHeap, HashMap},
    ops::Add,
};

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

    /*fn convert_to_grid(&self) -> Vec<Vec<i32>> {
        let mut indexer = HashMap::new();
        for line in self.input.iter() {
            let name = line.split(" ").collect::<Vec<&str>>()[0];

            let indexer_len = indexer.len();
            indexer.entry(name).or_insert(indexer_len);
        }

        let mut vec = vec![vec![0; indexer.len()]; indexer.len()];
        for line in self.input.iter() {
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
            vec[idx1][idx2] = num;
        }

        vec
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

    
    fn maximize_happiness(&self, grid: &Vec<Vec<i32>>) -> i32 {
        let mut vec: Vec<Paths> = Vec::new();
        for i in 0..grid.len() {
            let mut bitmask = vec![false; grid.len()];
            bitmask[i] = true; 

            let mut path = Paths::new();
            path.add(bitmask, 0, i, i);
            vec.push(path);
        }

        let mut dists: Vec<i32> = Vec::new(); 
        for paths in vec.iter_mut() {        
            dists.push(paths.main_logic(grid));
        }

        dists.sort();
        *dists.last().unwrap()
    }*/

}

/*struct Paths {
    data: BinaryHeap<Path<i32>>
}
impl Paths {
    fn new(mut bitmask: Vec<bool>, total_dist: i32, current_idx: usize, start_idx: usize) 
    -> Self {
        let mut data = BinaryHeap::new();
        data.push(
            Path::new(bitmask, total_dist, current_idx, start_idx)
        );
        Self { data }
    }


    fn main_logic(&mut self, grid: &Vec<Vec<i32>>) -> i32 {
        while let Some(path) = self.data.pop() {
            //checks if bitmask is filled then exit
            if path.is_bitmask_filled() {
                return path.total_dist;
            }

            for (idx, cities_len) in grid[path.current_idx].iter().enumerate() {
                //checking bitmask
                if !path.bitmask[idx] {
                    //adding path with updated bitmask
                    let bitmask = path.bitmask.clone();
                    let dist = self.calculate_total_dist(&path, *cities_len, grid);

                    self.add(bitmask, dist, idx, path.start_idx);
                }
            }
        }
        unreachable!()
    }

    fn calculate_total_dist(&self, path: &Path<i32>, len: i32, grid: &Vec<Vec<i32>>) -> i32 {
        let mut dist = path.total_dist;
        
        if path.is_bitmask_filled() {
            dist += grid[path.current_idx][path.start_idx];
        }

        dist + len
    }

    fn add(
        &mut self,
        mut bitmask: Vec<bool>,
        total_dist: i32,
        current_idx: usize,
        start_idx: usize,
    ) {
        bitmask[current_idx] = true;

        self.data.push(Path::new(bitmask, total_dist, current_idx, start_idx));
    }
}*/

impl Solution for Day13 {
    fn part1(&self) -> String {
        //let grid = self.convert_to_grid();

        //let most_happiness = self.maximize_happiness(&grid);
        let most_happiness = "";
        most_happiness.to_string()
    }
    fn part2(&self) -> String {
        format!("")
    }
}



/*
#[derive(Eq, PartialEq)]
pub struct Path<T> {
    pub bitmask: Vec<bool>,
    pub total_dist: T,
    pub current_idx: usize,
    pub start_idx: usize,
}

impl<T: PartialOrd + Copy + std::fmt::Display> Path<T> {
    pub fn new(bitmask: Vec<bool>, total_dist: T, current_idx: usize, start_idx: usize) -> Self {
        Self {
            bitmask,
            total_dist,
            current_idx,
            start_idx,
        }
    }
    fn is_bitmask_filled(&self) -> bool {
        for bit in self.bitmask.iter() {
            if !bit {
                return false;
            }
        }
        true
    }
    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "Path:\nBitmask: {:?}\nTotal dist: {}\nCurrent: {}",
            self.bitmask, self.total_dist, self.current_idx
        )
    }
}

impl<T> PartialOrd for Path<T>
where
    T: PartialEq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.total_dist.cmp(&other.total_dist))
    }
}

impl<T> Ord for Path<T>
where
    T: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .total_dist
            .cmp(&self.total_dist)
            .then_with(|| self.bitmask.cmp(&other.bitmask))
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
    /*#[test] fn test2() {
        let input = String::from("Alice would gain 54 happiness units by sitting next to Bob.\r\nAlice would lose 79 happiness units by sitting next to Carol.\r\nAlice would lose 2 happiness units by sitting next to David.\r\nBob would gain 83 happiness units by sitting next to Alice.\r\nBob would lose 7 happiness units by sitting next to Carol.\r\nBob would lose 63 happiness units by sitting next to David.\r\nCarol would lose 62 happiness units by sitting next to Alice.\r\nCarol would gain 60 happiness units by sitting next to Bob.\r\nCarol would gain 55 happiness units by sitting next to David.\r\nDavid would gain 46 happiness units by sitting next to Alice.\r\nDavid would lose 7 happiness units by sitting next to Bob.\r\nDavid would gain 41 happiness units by sitting next to Carol.");
        let day = Day13::new(input);

        assert_eq!(day.part2(), "330".to_string());
    }*/
}
*/