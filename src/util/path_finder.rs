use std::ops::Add;

pub trait Path<T>
where
    T: Ord + Copy + Add<Output = T> + std::fmt::Display + Default,
{
    fn create_new(&self, bitmask: Vec<bool>, idx: usize) -> Box<dyn Path<T>>;
    fn from_existing(&self, path_from: &Box<dyn Path<T>>, idx: usize, value: T) -> Box<dyn Path<T>>;
    fn get_total_dist(&self) -> T;
    fn get_current_idx(&self) -> usize;
    fn get_bitmask(&self) -> &Vec<bool>;
    fn get_bitmask_entry(&self, idx:usize) -> Option<&bool>; 
    fn get_initial(&self) -> usize { 0 }
    fn check_fulfillment_criteria(&self) -> bool;
    fn calculate_dist(&self, grid: &Vec<Vec<T>>, idx: usize) -> T;
    fn print(&self) {}
}

pub struct PathFinder<T> {
    data: Vec<Box<dyn Path<T>>>,
    path_type: Box<dyn Path<T>>,
}

impl<T> PathFinder<T>
where
    T: Ord + Copy + Add<Output = T> + std::fmt::Display + Default + 'static,
{
    pub fn new(path_type: impl Path<T> + 'static) -> Self {
        PathFinder {
            data: Vec::new(),
            path_type: Box::new(path_type)
        }
    }

    fn add(&mut self, path: &Box<dyn Path<T>>, idx: usize, total_dist: T) {
        let path = self.path_type.from_existing(path, idx, total_dist);
        self.data.push(path);
    }

    fn sort(&mut self, smallest: bool) {
        //let mut vec: Vec<Box<dyn Path<T>>> = self.data.clone().into();
        let comparator = match smallest {
            true => |b: &Box<dyn Path<T>>, a: &Box<dyn Path<T>>| {
                let mut dif = a.get_total_dist().cmp(&(b.get_total_dist()));
                if std::cmp::Ordering::Equal == dif {
                    dif = b
                        .get_bitmask()
                        .iter()
                        .filter(|bit| **bit)
                        .count()
                        .cmp(&a.get_bitmask().iter().filter(|bit| **bit).count());
                }
                dif
            },
            false => |a: &Box<dyn Path<T>>, b: &Box<dyn Path<T>>| {
                let mut dif = a.get_total_dist().cmp(&(b.get_total_dist()));
                if std::cmp::Ordering::Equal == dif {
                    dif = b
                        .get_bitmask()
                        .iter()
                        .filter(|bit| **bit)
                        .count()
                        .cmp(&a.get_bitmask().iter().filter(|bit| **bit).count());
                }
                dif
            },
        };

        self.data.sort_by(comparator);
    }


    #[allow(dead_code)]
    fn print(&self) {
        println!("-------------------- Paths --------------------");
        for path in self.data.iter() {
            path.print();
        }
    }

    fn find_path(&mut self, grid: &Vec<Vec<T>>, smallest: bool) -> Box<dyn Path<T>> {
        while let Some(path) = self.data.pop() {
            //checks if bitmask is filled then exit
            if path.check_fulfillment_criteria() { return path }

            //checking bitmask
            for (idx, _) in grid[path.get_current_idx()].iter().enumerate()
            .filter(|(idx, _)| path.get_bitmask_entry(*idx).is_some_and(|val| !val)) {
                //adding path with where bitmask is updated
                self.add(&path, idx, path.calculate_dist(grid, idx));
            }
            
            //Sort the paths such that the smallest value is first
            self.sort(smallest);
        }
        unreachable!()
    }

    pub fn route(&mut self, grid: Vec<Vec<T>>, smallest: bool) -> T {
        let mut largest_dist: Vec<T> = Vec::new();
        let len = grid.len();

        for i in 0..len {
            let path = 
            self.path_type.create_new(vec![false; len], i);
            self.data.push(path);
            largest_dist.push(self.find_path(&grid, smallest).get_total_dist());
            self.data.clear()
        }
        largest_dist.sort();

        if smallest {
            *largest_dist.first().unwrap()
        } else {
            *largest_dist.last().unwrap()
        }
    }
}
