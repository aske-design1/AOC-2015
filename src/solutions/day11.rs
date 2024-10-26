use super::*;

#[allow(dead_code)]
pub struct Day11 {
    input: String
}

impl Day11 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

pub struct Password(Vec<u8>);
impl Password {
    fn new(password: Vec<u8>) -> Self { Password(password) }
    fn increment(&mut self, idx: usize) {
        if let Some(val) = self.0.get_mut(idx) {
            *val = (((*val - b'a') + 1) % 26) + b'a';
            if *val == b'a' {
                self.increment(idx - 1);
            }
        }
    }
    fn new_password(&mut self) {
        while !self.is_accepted() {
            self.check_forbidden_chars();
            self.increment(self.get_last());
        }
    }
    fn get_last(&self) -> usize { self.0.len() - 1 }

    fn check_forbidden_chars(&mut self) {
        let mut found = false;
        for i in 0..self.0.len() {
            if found {
                self.0[i] = b'a'; 
            } else if self.0[i] == b'i' || self.0[i] == b'o' || self.0[i] == b'l' {
                self.increment(i);
                found = true; 
            }
        }
    }

    fn check_consecutive(&self, idx:usize) -> bool {
        if idx < 2 { return false } 
        
        let val1 = self.0[idx - 2];
        let val2 = self.0[idx - 1];
        let val3 = self.0[idx];

        val1 + 2 == val3 && val2 + 1 == val3
    }

    fn is_accepted(&self) -> bool {
        let mut three_consecutive = false;
        let mut two_consecutive = 0;
        let mut i = 0;
        let mut prev_pair_index = None;

        while i < self.0.len() {
            // Requirement 1: Check for three consecutive sequence
            if !three_consecutive && self.check_consecutive(i) {
                three_consecutive = true;
            }

            // Requirement 2: Check for forbidden characters `i`, `o`, `l`
            match self.0[i] {
                b'i' | b'o' | b'l' => return false,
                _ => (),
            };

            // Requirement 3: Check for two consecutive equal characters
            // Only count this pair if it doesn’t overlap with the previous counted pair
            if i >= 1 && self.0[i - 1] == self.0[i] && prev_pair_index != Some(i - 1) {
                two_consecutive += 1;
                prev_pair_index = Some(i);
            }
            
            i += 1;
        }
        two_consecutive >= 2 && three_consecutive
    }

    fn get_password<'a>(&'a self) -> &'a Vec<u8> {
        &self.0
    }

}


impl Solution for Day11 {
    type Output = String;
    fn part1(&self) -> String { 
        //Interval 97-122
        let mut password = Password::new(self.input.clone().as_bytes().to_vec());
        password.new_password();
        format!("{}", String::from_utf8(password.get_password().to_vec()).unwrap()) 
    }
    fn part2(&self) -> String { 
        let mut password = Password::new(self.input.clone().as_bytes().to_vec());
        password.new_password();
        password.increment(password.get_last());
        password.new_password();

        format!("{}", String::from_utf8(password.get_password().to_vec()).unwrap()) 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test1() {
        let input = "abcdefgh".to_string();
        let day = Day11::new(input);
        assert_eq!(day.part1(), "abcdffaa".to_string());

        
    }
    #[test] fn test2() {
        let input = "ghijklmn".to_string();
        let day = Day11::new(input);
        assert_eq!(day.part1(), "ghjaabcc".to_string());
    }
}