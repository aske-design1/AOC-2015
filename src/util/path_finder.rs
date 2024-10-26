use std::{
    collections::VecDeque, 
    ops::Add
};

#[derive(Clone)]
pub struct Path<T> {
    bitmask: Vec<bool>,
    total_dist: T,
    current: usize
}

impl<T: PartialOrd + Copy + std::fmt::Display> Path<T> {
    pub fn new(bitmask: Vec<bool>, total_dist: T, current: usize) -> Self {
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

pub struct Paths<T>(VecDeque<Path<T>>);
impl<T> Paths<T> 
where
    T: Ord + Copy + Add<Output = T> + std::fmt::Display + Default
{
    pub fn new() -> Self {
        Paths(VecDeque::new())
    }

    fn add(&mut self, mut bitmask: Vec<bool>, idx: usize, total_dist: T) {
        bitmask[idx] = true; 
        let path = Path::new(bitmask, total_dist, idx);
        self.0.push_back(path);
    }

    fn pop(&mut self, front: bool) -> Option<Path<T>> {
        match front {
            true => self.0.pop_front(),
            false => self.0.pop_back(),
        }
    }

    fn sort(&mut self) {
        let mut vec: Vec<Path<T>> = self.0.clone().into();
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

    fn find_path(&mut self, grid: &Vec<Vec<T>>, smallest: bool) -> Path<T> {
        
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
                    self.add(bitmask, idx, *cities_len + path.total_dist);
                }
            }
            
            //Sort the paths such that the smallest value is first
            self.sort();
        }

        unreachable!()
    }

    pub fn smallest_route(&self, cities: Vec<Vec<T>>) -> T {
        let mut paths = Paths::<T>::new();
        
        let len = cities.len();
        for i in 0..len {
            let bitmask = vec![false; len];
            paths.add(bitmask, i, T::default());
        }
        paths.find_path(&cities, true).total_dist
    }
    
    pub fn largest_route(&self, cities: Vec<Vec<T>>) -> T {
        let mut paths_to_check = Paths::new();
        
        let len = cities.len();
        for i in 0..len {
            let bitmask = vec![false; len];
            paths_to_check.add(bitmask, i, T::default());
        }

        let mut largest_dist: Vec<T> = Vec::new();
        while let Some(path_to_check) = paths_to_check.pop(false) {
            let mut paths = Paths::new();
            paths.add(path_to_check.bitmask, path_to_check.current, path_to_check.total_dist);
            largest_dist.push(paths.find_path(&cities, false).total_dist);
        }
        largest_dist.sort();
        *largest_dist.last().unwrap()
    }

}