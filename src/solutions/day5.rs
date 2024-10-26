use super::Solution;

pub struct Day5 {
    input: Vec<String>
}

impl Day5 {
    pub fn new(input: String) -> Self {
        let processed_input = input
        .split("\r\n")
        .filter(|line| *line != "")
        .map(|line| String::from(line))
        .collect();
        
        Self { input: processed_input }
    }

    fn is_nice(line: &str) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let bad_str = ["ab", "cd", "pq", "xy"];
        let mut vowel_amount = 0u32; 
        let mut double_letter = false; 
        for window in line.chars().collect::<Vec<char>>().windows(2) {
            //Check for first two rules
            if vowels.contains(&window[0]) {
                vowel_amount += 1; 
            };
            double_letter = double_letter || (window[0] == window[1]);

            let slice = format!("{}{}", window[0], window[1]);
            if bad_str.contains(&(slice.as_str())) {
                return false;
            }
        }
        if vowels.contains(&(line.chars().last().unwrap())) {
            vowel_amount += 1; 
        }

        double_letter && vowel_amount >= 3
    }

    fn is_nice2(line: &str) -> bool {
        let pair_of_2 = || {
            for i in 0..(line.len() - 1) {
                let slice = &line[i..i+2];
                for j in 0..(line.len() - 1) {
                    if j >= i && j <= i + 1 || j+1 >= i && j+1 <= i+1 { continue; }
                    if slice == &line[j..j+2] {
                        return true;
                    }
                }
            }
            false
        };
        let between = || {
            line.chars().collect::<Vec<char>>()
            .windows(3).any(|window| window[0] == window[2])
        };

        pair_of_2() && between()
    }


}

impl Solution for Day5 {
    type Output = String;

    fn part1(&self) -> String {        
        self.input.iter().filter(|line| Self::is_nice(line)).count().to_string()
    }
    fn part2(&self) -> String {
        self.input.iter().filter(|line| Self::is_nice2(line)).count().to_string()
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = Day5::new("ugknbfddgicrmopn\r\naaa\r\njchzalrnumimnmhp\r\nhaegwjzuvuyypxyu\r\ndvszwmarrgswjxmb"
            .to_string()
        );
        assert_eq!(test.part2().parse::<u32>().unwrap(), 2);
    }
    #[test]
    fn test2() {
        let test = Day5::new("dvszwmarrgswjxmb".to_string());
        assert_eq!(test.part2().parse::<u32>().unwrap(), 0);
    }

    #[test]
    fn test3() {
        let test = Day5::new("qjhvhtzxzqqjkmpb\r\nxxyxx\r\nuurcxstgmygtbstg\r\nieodomkazucvgmuy".to_string());
        assert_eq!(test.part2().parse::<u32>().unwrap(), 2);
    }

    #[test]
    fn test4() {
        let test = Day5::new("xxyxx".to_string());
        assert_eq!(test.part2().parse::<u32>().unwrap(), 1);
    }

    #[test] 
    fn test5() {
        assert_eq!(true, Day5::is_nice2("aadcdefgaa"));
        assert_eq!(true, Day5::is_nice2("xyxy"));
    }

    #[test] 
    fn test6() {
        assert_eq!(true, Day5::is_nice2("aaaa"));
        //println!("next");
        assert_eq!(false, Day5::is_nice2("aaa"));
    }

}