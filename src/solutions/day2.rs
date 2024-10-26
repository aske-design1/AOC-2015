use super::Solution;

pub struct Day2 {
    input: Vec<String>
}

impl Day2 {
    pub fn new(input: &str) -> Self {
        let processed_input: Vec<String> = input
        .split("\r\n")
        .filter(|el| *el != "")
        .map(|str| str.to_string())
        .collect();

        Self { input: processed_input }
    }

    fn calc_wrap(line: &str) -> u32 {
        let mut dimensions: Vec<u32> = line
        .split("x")
        .map(|el| el.parse::<u32>().unwrap())
        .collect();
        dimensions.sort();

        2 * dimensions[0] * dimensions[1] 
        + 2 * dimensions[1] * dimensions[2] 
        + 2 * dimensions[2] * dimensions[0]
        + dimensions[0] * dimensions[1]
    }

    fn calc_rib(line: &str) -> u32 {
        let mut dimensions: Vec<u32> = line
        .split("x")
        .map(|el| el.parse::<u32>().unwrap())
        .collect();
        dimensions.sort();

        //2+2+3+3 = 10
        // 2*3*4 = 24
        dimensions[0]*2 + dimensions[1]*2
        + dimensions[0] * dimensions[1] * dimensions[2]
    }

}

impl Solution for Day2 {
    type Output = String;
    fn part1(&self) -> String {
        let mut total = 0u32;
        for str in self.input.iter() {
            total += Self::calc_wrap(str); 
        }
        
        format!("{}", total)
    }
    fn part2(&self) -> String {
        let mut total = 0u32;
        for str in self.input.iter() {
            total += Self::calc_rib(str); 
        }
        
        format!("{}", total)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let day = Day2::new("2x3x4\r\n1x1x10");
        assert_eq!(day.part1().as_str(), "101");
    }
    #[test]
    fn test2() {
        let day = Day2::new("2x3x4\r\n1x1x10");
        assert_eq!(day.part2().as_str(), "48");
    }
    
}