use std::{
    clone, collections::VecDeque, ops::Add
};


pub trait PathData<T> {
    fn total_dist(&self) -> T;
    fn bitmask(&self) -> &Vec<bool>;
    fn get_bitmask_element(&self, idx: Option<usize>) -> bool;
    fn is_bitmask_filled(&self) -> bool;
    fn current(&self) -> usize;
}

pub trait PathList<T: PathData<U>, U> {
    fn get(&mut self) -> &mut Self;
    fn pop_front(&mut self) -> Option<T>;
    fn pop_back(&mut self) -> Option<T>;
}
//pub struct PathList<T>(VecDeque<T>);
pub trait Paths<T,U, V>
where
    T: Clone + PathData<U>,
    U: PartialEq + std::fmt::Display + Default + Clone + Ord + Add,
    V: PathList<T, U>
{
    type Output;
    /*fn add(&mut self, mut bitmask: Vec<bool>, idx: usize, total_dist: T) {
        bitmask[idx] = true; 
        let path = T::new(bitmask, total_dist, idx);
        self.0.push_back(path);
    }*/
    fn new(structure: V) -> Self::Output;
    fn pop_front(&mut self) -> Option<T> { V.pop_front() }
    fn pop_back(&mut self) -> Option<T> { V.pop_back() }
    fn sort(&mut self) {
        let mut vec: Vec<T> = self.0.clone().into();
        vec.sort_by(|a, b| {
            let mut dif = a.total_dist().cmp(&(b.total_dist()));
            if std::cmp::Ordering::Equal == dif {
                dif = b.bitmask().iter().filter(|bit| **bit).count()
                .cmp(&a.bitmask().iter().filter(|bit| **bit).count());
            }
            dif
        });

        self.0 = VecDeque::from(vec);
    }
    #[allow(dead_code)]
    /*fn print(&self) {
        println!("-------------------- Paths --------------------");
        for path in self.0.iter() {
            path.print();
        }
    }*/
    fn find_path(&mut self, grid: &Vec<Vec<U>>, smallest_dist: bool) -> T {
        while let Some(path) = 
        if smallest_dist { self.pop_front() } else { self.pop_back() } {
            
            //checks if bitmask is filled then exit
            if path.is_bitmask_filled() {
                return path
            }

            for (idx, cities_len) in grid[path.current()].iter().enumerate() {
                //checking bitmask
                if !path.get_bitmask_element(Some(idx)) {
                    //adding path with updated bitmask
                    let bitmask = path.bitmask().clone();
                    self.add(bitmask, idx, *cities_len + path.total_dist());
                }
            }
            
            //Sort the paths such that the smallest value is first
            self.sort();
        }

        unreachable!()
    }
    pub fn smallest_route(&self, grid: Vec<Vec<U>>) -> T {
        let mut paths = Paths::new();
        
        let len = grid.len();
        for i in 0..len {

            paths.add(len, i, U::default());
        }
        paths.find_path(&grid, true).total_dist
    }  
    pub fn largest_route(&self, grid: Vec<Vec<U>>) -> U {
        let mut paths_to_check = Paths::new();
        
        let len = grid.len();
        for i in 0..len {
            let bitmask = vec![false; len];
            paths_to_check.add(bitmask, i, U::default());
        }

        let mut largest_dist: Vec<U> = Vec::new();
        while let Some(path_to_check) = paths_to_check.pop_back() {
            let mut paths = Paths::new();
            paths.add(path_to_check.bitmask, path_to_check.current, path_to_check.total_dist);
            largest_dist.push(paths.find_path(&grid, false).total_dist);
        }
        largest_dist.sort();
        *largest_dist.last().unwrap()
    }
    fn add_to_list(&mut self, grid: Vec<Vec<T>>);
    fn add_new_element(&mut self);
}

