use std::collections::HashMap;

use super::*;

#[allow(dead_code)]
pub struct Day23 {
    input: Vec<String>
}

impl Day23 {
    pub fn new(input: String) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let input = input.split(splitter).map(|line| line.to_string()).collect();
        Self { input }
    }
}

impl Day23 {
    fn line_parser(line: &str, registers: &mut HashMap<&str, u32>, mut idx: usize) -> usize {
        let (command, update) = line.split_once(" ").unwrap();
        
        idx = match command {
            "hlf" => {
                if let Some(val) = registers.get_mut(update) { *val /=2 }  
                idx + 1
            },
            "tpl" => {
                if let Some(val) = registers.get_mut(update) { *val *= 3 }  
                idx + 1
            },
            "inc" => {
                if let Some(val) = registers.get_mut(update) { *val += 1 }    
                idx + 1
            },
            "jmp" => {
                let update_num = &update[1..].parse::<usize>().unwrap();
                if update.contains("+") { idx + *update_num } 
                else { idx - *update_num }
            }, 
            "jie" | "jio" => {
                let (update, jump_with) = update.split_once(", ").unwrap(); 
                let update_num = &jump_with[1..].parse::<usize>().unwrap();

                match (command, registers.get(update)) {
                    ("jie", Some(val)) if *val % 2 == 0 => {
                        if jump_with.contains("+") { idx + *update_num } 
                        else { idx - *update_num }
                    },
                    ("jio", Some(val)) if *val == 1 => {
                        if jump_with.contains("+") { idx + *update_num } 
                        else { idx - *update_num }
                    }
                    (_, _) => idx + 1
                }
            },
            _ => unreachable!()
        };

        idx
    }
    fn main_logic(&self, start_val: u32) -> u32 { 
        let mut registers = HashMap::new();
        registers.insert("a", start_val);
        registers.insert("b", 0);
        let mut i = 0;
        while i < self.input.len() {
            i = Self::line_parser(&self.input[i], &mut registers, i);
        }
        *registers.get("b").unwrap()
    }


}

impl Solution for Day23 {
    fn part1(&self) -> String { 
        self.main_logic(0).to_string() 
    }
    fn part2(&self) -> String { 
        self.main_logic(1).to_string() 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test1() {
        let mut hash = HashMap::new();
        assert_eq!(23, Day23::line_parser("jmp +23", &mut hash, 0));
        assert_eq!(5, Day23::line_parser("jio a, +119", &mut hash, 4));
    }
    //#[test] fn test2() {}
}