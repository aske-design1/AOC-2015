use std::{cmp::Reverse, collections::BinaryHeap};

use super::*;

#[allow(dead_code)]
pub struct Day24 {
    weights: Vec<u32>,
    total_sum: u32
}

impl Day24 {
    pub fn new(input: String) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };

        let weights: Vec<u32> = input.split(splitter).map(|line| line.parse().unwrap()).collect();
        let total_sum = weights.iter().sum::<u32>();
        
        Self { 
            weights,
            total_sum
        }
    }
    #[allow(dead_code)]
    fn new_test(weights: Vec<u32>) -> Self {
        let total_sum = weights.iter().sum::<u32>();
        Self { 
            weights,
            total_sum
        }        
    }

    fn main_logic(&self, groups: u32) -> u64 {
        let sorted_arr = self.weights.clone();
        let mut min_heap:BinaryHeap<Reverse<(usize, Vec<u64>)>> = BinaryHeap::new();

        Self::helper(&sorted_arr, &mut min_heap, &(self.total_sum / groups), Vec::new(), 0);
        
        let Reverse(tuple) = min_heap.pop().unwrap();

        let mut smallest_quan: u64 = tuple.1.iter().product();

        while let Some(Reverse(el)) = min_heap.pop() {
            let quantam = el.1.iter().product::<u64>();
            if tuple.0 != el.0 {
                break; 
            } else if quantam < smallest_quan {
                smallest_quan = quantam;
            }
        }
        smallest_quan
    }

    fn helper(weights: &Vec<u32>, heap: &mut BinaryHeap<Reverse<(usize, Vec<u64>)>>, distribution: &u32, cur_weight: Vec<u64>, idx: usize) {
        let collective_weight = cur_weight.iter().sum::<u64>() as u32;
        if collective_weight == *distribution {
            return heap.push(Reverse((cur_weight.len(), cur_weight)))
        } else if collective_weight > *distribution || 
        idx >= weights.len() || 
        heap.peek().is_some_and(|Reverse(val)| val.1.len() < cur_weight.len()) {
            return
        }

        let mut next_weight = cur_weight.clone();
        next_weight.push(weights[idx] as u64);

        Self::helper(weights, heap, distribution, cur_weight.clone(), idx + 1);
        Self::helper(weights, heap, distribution, next_weight, idx + 1);
    }

}

impl Solution for Day24 {
    fn part1(&self) -> String { 
        self.main_logic(3).to_string() 
    }
    fn part2(&self) -> String { 
        self.main_logic(4).to_string()  
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test1() {
        let weights: Vec<u32> = vec![1,2,3,4,5,7,8,9,10,11];
        let day = Day24::new_test(weights);
        assert_eq!(day.part1(), 99.to_string())
    }
    #[test] 
    fn test2() {
        let weights: Vec<u32> = vec![1,2,3,4,5,7,8,9,10,11];
        let day = Day24::new_test(weights);
        assert_eq!(day.part2(), 44.to_string())
    }
}