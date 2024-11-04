use std::{collections::{HashMap, HashSet}, u32};

use super::*;

pub struct Day19 {
    replacements: HashMap<String, Vec<String>>,
    molecules: String,
    reverse_replacements: Vec<(String, String)>
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
        let mut rev_replacements = Vec::new();
        for replacement in replacements.split(splitter) {
            let (replace, replace_with) = replacement.split_once(" => ").unwrap();

            //println!("{replacement}");
            rev_replacements
            .push((replace_with.to_string(), replace.to_string()));

            replacements_hash
            .entry(replace.to_string())
            .and_modify(|vec: &mut Vec<String>| vec.push(replace_with.to_string()))
            .or_insert(vec![replace_with.to_string()]);
        }
        let molecules = molecules.to_string();

        //println!("{:?}", replacements_hash);
        Self {
            molecules,
            replacements: replacements_hash,
            reverse_replacements: rev_replacements,
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
                    distinct.insert(format!("{}{}{}", &self.molecules[..i], replacement, &self.molecules[i+1..]));
                }
            } 
            
            if i < self.molecules.len() - 1 {
                let sec_slice = &self.molecules[i..i + 2];
                if let Some(replacements) = self.replacements.get(sec_slice) {
                    for replacement in replacements {
                        distinct.insert(format!("{}{}{}", &self.molecules[..i], replacement, &self.molecules[i+2..]));
                    }
                    i+=2;
                    continue;
                }
            } 

            i += 1;
        }

        distinct.len()
    }
    
    //Idea: Go from molecule -> "e"
    // Potentially doesnt work for all inputs
    fn make_molecule(&self) -> u32 {
        let mut reverse_replacements = self.reverse_replacements.clone();
        reverse_replacements.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

        let mut steps = 0;
        let mut molecule_str = self.molecules.clone();

        while molecule_str != "e" {    
            for (key, value) in reverse_replacements.iter().rev() {
                if let Some(pos) = molecule_str.find(key) {
                    let temp_str = 
                    format!("{}{}{}", &molecule_str[..pos], value, &molecule_str[pos+key.len()..]);
                    
                    if temp_str.len() <= molecule_str.len() {
                        molecule_str = temp_str;
                        steps+=1;
                        break;
                    }
                    
                }
            }
        }

        steps
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
