use std::{collections::{HashMap, HashSet}, u32};

use super::*;

#[allow(dead_code)]
pub struct Day19 {
    replacements: HashMap<String, Vec<String>>,
    molecules: String,
}

impl Day19 {
    pub fn new(input: String) -> Self {
        let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
        let sec_splitter = if input.contains("\r\n\r\n") {
            "\r\n\r\n"
        } else {
            "\n\n"
        };
        let (replacements, molecules) = input.split_once(sec_splitter).unwrap();

        let mut replacements_hash = HashMap::new();
        for replacement in replacements.split(splitter) {
            let (replace, replace_with) = replacement.split_once(" => ").unwrap();

            //println!("{replacement}");

            replacements_hash
                .entry(replace.to_string())
                .and_modify(|vec: &mut Vec<String>| vec.push(replace_with.to_string()))
                .or_insert(vec![replace_with.to_string()]);
        }
        let molecules = molecules.to_string();

        //println!("{:?}", replacements_hash);
        Self {
            replacements: replacements_hash,
            molecules,
        }
    }

    fn one_modification(&self) -> usize {
        let mut distinct: HashSet<String> = HashSet::new();

        let mut i = 0;
        while i < self.molecules.len() {
            let slice = &self.molecules[i..i + 1];
            //logic
            if let Some(replacements) = self.replacements.get(slice) {
                for replacement in replacements {
                    distinct.insert(self.molecules[..i].to_string() + replacement.as_str() + &self.molecules[i+1..]);
                }
            } 
            
            if i <= self.molecules.len() - 2 {
                let sec_slice = &self.molecules[i..i + 2];
                if let Some(replacements) = self.replacements.get(sec_slice) {
                    for replacement in replacements {
                        distinct.insert(self.molecules[..i].to_string() + replacement.as_str() + &self.molecules[i+2..]);
                    }
                    i+=2;
                    continue;
                }
            } 

            i += 1;
        }

        distinct.len()
    }

    fn make_molecule(&self) -> u32 {
        let mut steps = u32::MAX;
        let init_str = "e".to_string();

        for replacement in self.replacements.get(&init_str).unwrap() {
            //println!("{replacement}");
            steps = steps.min(self.make_molecule_helper(1, &replacement));
        }

        steps
    }

    fn make_molecule_helper(&self, steps: u32, str: &str) -> u32 {
        if self.molecules == str || 
        str.len() > self.molecules.len() { return steps }
        //println!("{str}");


        let mut min_steps = u32::MAX; 
        let mut i = 0;
        while i < str.len() {
            let slice = &str[i..i + 1];
            //logic
            if let Some(replacements) = self.replacements.get(slice) {
                for replacement in replacements {
                    let new_str = str[..i].to_string() + replacement.as_str() + &str[i+1..];
                    min_steps = min_steps.min(self.make_molecule_helper(steps + 1, &new_str));
                }
            } 
            
            if i <= str.len() - 2 {
                let sec_slice = &str[i..i + 2];
                if let Some(replacements) = self.replacements.get(sec_slice) {
                    for replacement in replacements {
                        let new_str = str[..i].to_string() + replacement.as_str() + &str[i+2..];
                        min_steps = min_steps.min(self.make_molecule_helper(steps + 1, &new_str));
                    }
                    i+=2;
                    continue;
                }
            } 

            i += 1;
        }
        min_steps
    }


    fn find_distinct(&self) -> usize {
        let mut distinct: HashSet<String> = HashSet::new();

        //println!("{:?}", self.molecules);

        self.find_distinct_helper(&mut distinct, 0, "");

        for el in distinct.iter() {
            println!("{el}");
        }

        distinct.len()
    }

    fn find_distinct_helper<'a>(
        &'a self,
        distinct: &mut HashSet<String>,
        idx: usize,
        new_dna: &str,
    ) {
        if idx == self.molecules.len() {
            distinct.insert(new_dna.to_string());
            return;
        }

        //println!("string: {}, idx: {idx}", new_dna);
        println!("{:?}", self.replacements);

        let slice = &self.molecules[idx..idx + 1];
        //logic
        if let Some(replacements) = self.replacements.get(slice) {
            for replacement in replacements {
                let new_dna = new_dna.to_string() + replacement.as_str();
                self.find_distinct_helper(distinct, idx + 1, new_dna.as_str());
            }
        } else if idx <= self.molecules.len() - 1
            && self
                .replacements
                .contains_key(&self.molecules[idx..idx + 2])
        {
            let sec_slice = &self.molecules[idx..idx + 2];
            let replacements = self.replacements.get(sec_slice).unwrap();
            for replacement in replacements {
                let new_dna = new_dna.to_string() + replacement.as_str();
                self.find_distinct_helper(distinct, idx + 2, new_dna.as_str());
            }
        } else {
            let new_dna = new_dna.to_string() + slice;
            self.find_distinct_helper(distinct, idx + 1, new_dna.as_str())
        }
    }
}

impl Solution for Day19 {
    fn part1(&self) -> String {
        self.one_modification().to_string()
    }
    fn part2(&self) -> String {
        self.make_molecule().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test1() {
        let input = "H => HO\nH => OH\nO => HH\n\nHOH";
        let day = Day19::new(input.to_string());

        assert_eq!(day.part1(), 4.to_string())
    }
    #[test]
    fn part_one_test2() {
        let input = "H => HO\nH => OH\nO => HH\n\nHOHOHO";
        let day = Day19::new(input.to_string());

        assert_eq!(day.part1(), 7.to_string())
    }


    #[test]
    fn part_two_test1() {
        let input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH";
        let day = Day19::new(input.to_string());

        assert_eq!(day.part2(), 3.to_string())
    }
    #[test]
    fn part_two_test2() {
        let input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO";
        let day = Day19::new(input.to_string());

        assert_eq!(day.part2(), 6.to_string())
    }
}
