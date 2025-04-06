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
            aunt.create_iter().zip(criteria.create_iter()).all(|(aunt, criteria)| {
                Self::check_field(aunt, criteria)
            })
            /*Self::check_field(aunt.children, criteria.children) &&
            Self::check_field(aunt.cats, criteria.cats) &&
            Self::check_field(aunt.samoyeds, criteria.samoyeds) &&
            Self::check_field(aunt.pomeranians, criteria.pomeranians) &&
            Self::check_field(aunt.akitas, criteria.akitas) &&
            Self::check_field(aunt.vizslas, criteria.vizslas) &&
            Self::check_field(aunt.goldfish, criteria.goldfish) &&
            Self::check_field(aunt.trees, criteria.trees) &&
            Self::check_field(aunt.cars, criteria.cars) &&
            Self::check_field(aunt.perfumes, criteria.perfumes)*/
        }
        fn check_aunt_with_ranges(aunt: &Aunt, criteria: &Aunt) -> bool {
            let (a_field, a_range) = aunt.create_range_iter_and_iter();
            let (c_field, c_range) = criteria.create_range_iter_and_iter();

            a_field.zip(c_field).all(|(aunt, criteria)| {
                Self::check_field(aunt, criteria)
            }) && a_range.zip(c_range).all(|((aunt, gt), (criteria, _))| {
                Self::check_range(aunt, criteria, gt) 
            })

            /*Self::check_field(aunt.children, criteria.children) &&
            Self::check_range(aunt.cats, criteria.cats, true) &&
            Self::check_field(aunt.samoyeds, criteria.samoyeds) &&
            Self::check_range(aunt.pomeranians, criteria.pomeranians, false) &&
            Self::check_field(aunt.akitas, criteria.akitas) &&
            Self::check_field(aunt.vizslas, criteria.vizslas) &&
            Self::check_range(aunt.goldfish, criteria.goldfish, false) &&
            Self::check_range(aunt.trees, criteria.trees, true) &&
            Self::check_field(aunt.cars, criteria.cars) &&
            Self::check_field(aunt.perfumes, criteria.perfumes)*/
        }

        fn check_field(aunt_field: Option<u32>, criteria_field: Option<u32>) -> bool {
            match (aunt_field, criteria_field) {
                (Some(aunt), Some(criteria)) => aunt == criteria,
                (_, _) => true,
            }

            /*if let Some(field) = aunt_field { field == criteria_field } 
            else { true }*/
        }
                
        fn check_range(aunt_field: Option<u32>, criteria_field: Option<u32>, greater_than: bool) -> bool {
            match (aunt_field, criteria_field) {
                (Some(aunt), Some(criteria)) => greater_than && aunt > criteria || aunt < criteria,
                (_, _) => true
            }

            /*if let Some(field) = aunt_field { 
                match greater_than {
                    true => field > criteria_field,
                    false => field < criteria_field
                }
            } 
            else { true }*/
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
        fn create_iter(&self) -> impl Iterator<Item = Option<u32>> {
            vec![self.children, self.cats, self.samoyeds, self.pomeranians, self.akitas, self.vizslas, self.goldfish, self.trees, self.cars, self.perfumes].into_iter()
        }
        fn create_range_iter_and_iter(&self) -> (impl Iterator<Item = Option<u32>>, impl Iterator<Item = (Option<u32>, bool)>) {
            ( 
                vec![self.children,  self.samoyeds,  self.akitas, self.vizslas,  self.cars, self.perfumes].into_iter(), 
                vec![(self.cats, true), (self.pomeranians, false), (self.goldfish, false), (self.trees, true)].into_iter()
            )
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