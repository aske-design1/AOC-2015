use super::Solution;

pub struct Day1 {
    input: String
}

impl Day1 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Solution for Day1 {
    type Output = String;
    fn part1(&self) -> String {
        format!("{}",self.input
        .chars()
        .fold(0, |accu, ch| 
            accu + 
            if ch == '(' { 1 } 
            else         { -1 }
        ))
    }
    fn part2(&self) -> String {
        let mut floor = 0i32;
        let idx = self.input.chars().position(|ch| {
            floor += if ch == '(' { 1 } 
            else { -1 };

            floor == -1
        });
        
        match idx {
            Some(i) => format!("{}", i+1),
            None => format!("{}", -1)
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = Day1::new("(())".to_string());
        assert_eq!(test.part1(), "-1".to_string());
    }
    #[test]
    fn test2() {
        let test = Day1::new("(()(()(".to_string());
        assert_eq!(test.part1(), "3".to_string());
    }
    #[test]
    fn test3() {
        let test = Day1::new("))(((((".to_string());
        assert_eq!(test.part1(), "2".to_string());
    }
    #[test]
    fn test4() {
        let test = Day1::new("())".to_string());
        assert_eq!(test.part1(), "-1".to_string());
    }
    #[test]
    fn test5() {
        let test = Day1::new(")())())".to_string());
        assert_eq!(test.part1(), "-3".to_string());
    }
    #[test]
    fn test6() {
        let test = Day1::new("()())".to_string());
        assert_eq!(test.part1(), "5".to_string());
    }
}