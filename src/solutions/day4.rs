use super::Solution;

pub struct Day4 {
    input: String
}

impl Day4 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
    fn check_hash(hash: &str, repeat: usize) -> bool {
        if hash.len() < repeat { return false; }
        &hash[..repeat] == "0".repeat(repeat)
    }

}

impl Solution for Day4 {
    fn part1(&self) -> String {
        let mut hash: String = String::from(""); 
        let mut num = 0u32; 
        while !Self::check_hash(&hash, 5) {
            num += 1;
            hash = format!("{:x}", 
                md5::compute(
                    format!("{}{}", self.input, num).as_bytes()
                )
            );
        }

        format!("{}", num)
    }
    fn part2(&self) -> String {
        let mut hash: String = String::from(""); 
        let mut num = 0u32; 
        while !Self::check_hash(&hash, 6) {
            num += 1;
            hash = format!("{:x}", 
                md5::compute(
                    format!("{}{}", self.input, num).as_bytes()
                )
            );
        }

        format!("{}", num)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = Day4::new("abcdef".to_string());
        assert_eq!(test.part1(), "609043".to_string());
    }
    #[test]
    fn test2() {}
}