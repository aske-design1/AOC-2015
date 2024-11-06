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
    /*fn find_iterations(row: usize, col: usize) -> usize {
        let mut iterations = 1usize;

        /*let (lowest, largest) = if row >= col {
            (col, row)
        } else { (row, col) };*/

        for i in 1..row { iterations += i  }
        for i in 1..col { iterations += row + i }
        

        iterations
    }*/
    
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

    #[test] 
    fn test_iterations() {
        /*assert_eq!(Day25::find_iterations(1,1), 1);
        assert_eq!(Day25::find_iterations(2,1), 2);
        assert_eq!(Day25::find_iterations(1,2), 3);
        assert_eq!(Day25::find_iterations(3,1), 4);
        assert_eq!(Day25::find_iterations(2,2), 5);
        assert_eq!(Day25::find_iterations(1,3), 6);
        assert_eq!(Day25::find_iterations(4,1), 7);
        assert_eq!(Day25::find_iterations(3,2), 8);
        assert_eq!(Day25::find_iterations(2,3), 9);
        assert_eq!(Day25::find_iterations(1,4), 10);
        assert_eq!(Day25::find_iterations(5,1), 11);
        assert_eq!(Day25::find_iterations(4,2), 12);
        assert_eq!(Day25::find_iterations(3,3), 13);
        assert_eq!(Day25::find_iterations(2,4), 14);
        assert_eq!(Day25::find_iterations(1,5), 15);
        assert_eq!(Day25::find_iterations(6,1), 16);
        assert_eq!(Day25::find_iterations(5,2), 17);
        assert_eq!(Day25::find_iterations(4,3), 18);
        assert_eq!(Day25::find_iterations(3,4), 19);
        assert_eq!(Day25::find_iterations(2,5), 20);
        assert_eq!(Day25::find_iterations(1,6), 21);*/
    }
}