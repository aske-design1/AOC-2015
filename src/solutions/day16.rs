use std::collections::HashMap;

use super::*;

    #[allow(dead_code)]
    pub struct Day16 {
        input: Vec<Aunt>
    }

    impl Day16 {
        pub fn new(input: String) -> Self {
            let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
            let input = input
            .split(splitter)
            .map(|line| Self::parse_line(line))
            .collect::<Vec<Aunt>>();

            Self { input }
        }

        fn parse_line(line: &str) -> Aunt {
            let (aunt, characteristics) = line.split_once(": ").unwrap();
            let num = aunt.split_once(" ").unwrap().1.parse::<u32>().unwrap();

            let mut charactericts_hash: HashMap<&str, u32> = HashMap::new();

            for characteristic in characteristics.split(", ") {
                let (identifier, num) = characteristic.split_once(": ").unwrap();
                charactericts_hash.insert(identifier, num.parse::<u32>().unwrap());
            } 

            Aunt::from(num as usize, &charactericts_hash)
        }

        fn find_aunt(&self, criteria: Aunt, check_aunt: fn(&Aunt, &Aunt) -> bool) -> usize {
            let mut aunt = self.input.iter().filter(|aunt| check_aunt(*aunt, &criteria));
            aunt.next().unwrap().num_aunt
        }

        fn check_aunt(aunt: &Aunt, criteria: &Aunt) -> bool {
            Self::check_field(aunt.children, criteria.children.unwrap()) &&
            Self::check_field(aunt.cats, criteria.cats.unwrap()) &&
            Self::check_field(aunt.samoyeds, criteria.samoyeds.unwrap()) &&
            Self::check_field(aunt.pomeranians, criteria.pomeranians.unwrap()) &&
            Self::check_field(aunt.akitas, criteria.akitas.unwrap()) &&
            Self::check_field(aunt.vizslas, criteria.vizslas.unwrap()) &&
            Self::check_field(aunt.goldfish, criteria.goldfish.unwrap()) &&
            Self::check_field(aunt.trees, criteria.trees.unwrap()) &&
            Self::check_field(aunt.cars, criteria.cars.unwrap()) &&
            Self::check_field(aunt.perfumes, criteria.perfumes.unwrap())
        }
        fn check_aunt_with_ranges(aunt: &Aunt, criteria: &Aunt) -> bool {
            Self::check_field(aunt.children, criteria.children.unwrap()) &&
            Self::check_range(aunt.cats, criteria.cats.unwrap(), true) &&
            Self::check_field(aunt.samoyeds, criteria.samoyeds.unwrap()) &&
            Self::check_range(aunt.pomeranians, criteria.pomeranians.unwrap(), false) &&
            Self::check_field(aunt.akitas, criteria.akitas.unwrap()) &&
            Self::check_field(aunt.vizslas, criteria.vizslas.unwrap()) &&
            Self::check_range(aunt.goldfish, criteria.goldfish.unwrap(), false) &&
            Self::check_range(aunt.trees, criteria.trees.unwrap(), true) &&
            Self::check_field(aunt.cars, criteria.cars.unwrap()) &&
            Self::check_field(aunt.perfumes, criteria.perfumes.unwrap())
        }

        fn check_field(aunt_field: Option<u32>, criteria_field: u32) -> bool {
            if let Some(field) = aunt_field { field == criteria_field } 
            else { true }
        }
                
        fn check_range(aunt_field: Option<u32>, criteria_field: u32, greater_than: bool) -> bool {
            if let Some(field) = aunt_field { 
                match greater_than {
                    true => field > criteria_field,
                    false => field < criteria_field
                }
            } 
            else { true }
        }


    }

    struct Aunt {
        num_aunt: usize,
        children: Option<u32>,
        cats: Option<u32>,
        samoyeds: Option<u32>,
        pomeranians: Option<u32>,
        akitas: Option<u32>,
        vizslas: Option<u32>,
        goldfish: Option<u32>,
        trees: Option<u32>,
        cars: Option<u32>,
        perfumes: Option<u32>,
    }
    impl Aunt {
        fn from(num_aunt: usize, hash: &HashMap<&str, u32>) -> Self {
            Self {
                num_aunt,
                children: hash.get("children").copied(),
                cats :hash.get("cats").copied(),
                samoyeds: hash.get("samoyeds").copied(),
                pomeranians: hash.get("pomeranians").copied(),
                akitas: hash.get("akitas").copied(),
                vizslas: hash.get("vizslas").copied(),
                goldfish: hash.get("goldfish").copied(),
                trees: hash.get("trees").copied(),
                cars: hash.get("cars").copied(),
                perfumes: hash.get("perfumes").copied(),
            }
        }
    }

    impl Solution for Day16 {
        fn part1(&self) -> String { 
            let criteria = Aunt {
                num_aunt: 0,
                children: Some(3),
                cats: Some(7),
                samoyeds: Some(2),
                pomeranians: Some(3),
                akitas: Some(0),
                vizslas: Some(0),
                goldfish: Some(5),
                trees: Some(3),
                cars: Some(2),
                perfumes: Some(1)
            };
            self.find_aunt(criteria, Self::check_aunt).to_string() 
        }
        fn part2(&self) -> String { 
            let criteria = Aunt {
                num_aunt: 0,
                children: Some(3),
                cats: Some(7),
                samoyeds: Some(2),
                pomeranians: Some(3),
                akitas: Some(0),
                vizslas: Some(0),
                goldfish: Some(5),
                trees: Some(3),
                cars: Some(2),
                perfumes: Some(1)
            };
            self.find_aunt(criteria, Self::check_aunt_with_ranges).to_string() 
        } 
    }

    #[cfg(test)]
    mod tests {
        //use super::*;
        //#[test] fn test1() {}
        //#[test] fn test2() {}
    }