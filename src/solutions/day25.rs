use super::*;

#[allow(dead_code)]
pub struct Day25 {
    row: usize,
    col: usize
}

impl Day25 {
    pub fn new(input: String) -> Self {
        let splitter = "To continue, please consult the code grid in the manual.  Enter the code at row ";
        let (_, last) = input.split_once(splitter).unwrap();

        let (row, col) = last.split_once(", column ").unwrap();
        let row: usize = row.parse().unwrap();
        //'.' after the num
        let col: usize = col.split_once(".").unwrap().0.parse().unwrap();

        Self { row, col }
    }
    fn new_test(row:usize, col: usize) -> Self {
        Self { row, col }
    }
    fn main_logic(&self) -> u64 { 
        let mut row = 1; 
        let mut col = 1; 
        let mut code = 20151125;
        while row != self.row || col != self.col {
            if row == 1 {
                row = col + 1;
                col = 1;
            } else {
                row -= 1;
                col += 1;
            }
            code = Self::calculate_code(code);
            //if row == self.row && col == self.col { break }
        }
        code
    }
    fn calculate_code(code: u64) -> u64 {
        (code * 252533) % 33554393 
    }
    
}

impl Solution for Day25 {
    fn part1(&self) -> String { 
        self.main_logic().to_string() 
    }
    fn part2(&self) -> String { format!("Merry Christmas") } 
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test] 
    fn part_one_1_6() {
        let day = Day25::new_test(1, 6);
        assert_eq!(day.part1(), 33511524.to_string())
    }
    #[test]
    fn part_one_1_1() {
        let day = Day25::new_test(1, 1);
        assert_eq!(day.part1(), 20151125.to_string())
    }
    #[test]
    fn part_one_2_1() {
        let day = Day25::new_test(2, 1);
        assert_eq!(day.part1(), 31916031.to_string())
    }
}