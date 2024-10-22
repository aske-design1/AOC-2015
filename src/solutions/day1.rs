use super::Solution;

pub struct Day1<'a> {
    input: &'a str
}

impl<'a> Day1<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Solution<'a> for Day1<'a> {
    fn part1(&self) -> &'a str {
        "0"
    }
    fn part2(&self) -> &'a str {
        ""
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = Day1::new("(())");
        assert_eq!(test.part1(), "0");
    }
    #[test]
    fn test2() {
        let test = Day1::new("(()(()(");
        assert_eq!(test.part1(), "3");
    }
    #[test]
    fn test3() {
        let test = Day1::new("))(((((");
        assert_eq!(test.part1(), "3");
    }
    #[test]
    fn test4() {
        let test = Day1::new("())");
        assert_eq!(test.part1(), "-1");
    }
    #[test]
    fn test5() {
        let test = Day1::new(")())())");
        assert_eq!(test.part1(), "-3");
    }
}