use super::*;

    #[allow(dead_code)]
    pub struct Day18 {
        input: Vec<String>
    }

    impl Day18 {
        pub fn new(input: String) -> Self {
            let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
            let input = input.split(splitter).map(|line| line.to_string()).collect();
            Self { input }
        }

        fn empty_grid(from_grid:&Vec<Vec<u8>>) -> Vec<Vec<u8>> {
            let mut grid = Vec::new();
            for line in from_grid.iter() {
                let len = line.len();
                let byte_vec = vec![b'.'; len];
                grid.push(byte_vec);
            }
            grid
        }

        fn lights_on(
            steps: usize, 
            on_logic: for<'a> fn(u8, usize, &'a Point, col_len:usize, row_len:usize) -> bool,
            init_grid: Vec<Vec<u8>>
            ) -> u32 {
            let final_grid = (0..steps).fold(init_grid, |current_grid, _| Self::next_grid(current_grid, on_logic));

            final_grid.iter().flat_map(|el| el.iter()).filter(|el| **el == b'#').count() as u32
        }
        fn get_grid(&self, corner_lights: bool) -> Vec<Vec<u8>> {
            let mut grid: Vec<Vec<u8>> = self.input.iter().map(|line| line.as_bytes().to_vec()).collect();
            if corner_lights {
                let row_len = grid.len() - 1;
                let col_len = grid[0].len() - 1; 

                grid[0][0] = b'#';
                grid[0][col_len] = b'#';
                grid[row_len][0] = b'#';
                grid[row_len][col_len] = b'#';
            }
            grid
        }

        fn next_grid(
            grid: Vec<Vec<u8>>, 
            on_logic: for<'a> fn(u8, usize, &'a Point, col_len:usize, row_len:usize) -> bool) 
            -> Vec<Vec<u8>> {
            let mut cord_on: Vec<Point> = Vec::new();
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    let point = Point::from((j, i));
                    let neighbors = Self::get_neighbors(&point, &grid);
                    let count = neighbors.iter().filter(|el| **el == b'#').count();
                    if on_logic(grid[i][j], count, &point, grid[i].len() -1, grid.len() -1) { 
                        cord_on.push(point) 
                    }
                }
            }
            let mut new_grid = Self::empty_grid(&grid);        
            for point in cord_on {
                new_grid[point.y][point.x] = b'#' 
            }
            
            new_grid
        }

        fn get_neighbors(from: &Point, grid: &Vec<Vec<u8>>) -> Vec<u8> {
            let (x, y) = from.get_as_isize();
            let (row_len, col_len) = (grid.len(), grid[y as usize].len());

            let points = vec![
                Point::try_from((x-1, y-1)), //1
                Point::try_from((x, y-1)),   //2
                Point::try_from((x+1, y-1)), //3
                Point::try_from((x-1, y)),   //4
                Point::try_from((x+1, y)),   //5
                Point::try_from((x-1, y+1)), //6
                Point::try_from((x, y+1)),   //7
                Point::try_from((x+1, y+1)), //8
            ];
            points.into_iter()
            .filter(|point| point.is_ok() && point.as_ref().unwrap().within_range(col_len, row_len))
            .map(|el| el.unwrap())
            .map(|point| grid[point.y][point.x])
            .collect()
        }

    }

    struct Point {
        x: usize,
        y: usize
    }

    impl TryFrom<(isize, isize)> for Point {
        type Error = std::option::Option<String>;
        fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
            if value.0 < 0 || value.1 < 0 {
                Err(Some("Not valid".to_string()))
            } else {
                Ok(Point::from((value.0 as usize, value.1 as usize)))
            }

        }
    }

    impl From<(usize, usize)> for Point {
        fn from(point: (usize, usize)) -> Self {
            Self {
                x: point.0,
                y: point.1
            }
        }
    }
    impl Point {
        fn get_as_isize(&self) -> (isize, isize) {
            (self.x as isize, self.y as isize)
        }

        fn within_range(&self, x_range: usize, y_range:usize) -> bool {
            self.x < x_range  && self.y < y_range
        }

        fn check(&self, x:usize, y:usize) -> bool {
            self.x == x && self.y == y
        }
    }

    impl Solution for Day18 {
        fn part1(&self) -> String {
            let steps = 100usize;

            let on_logic: for<'b> fn(u8, usize, &'b Point, col_len:usize, row_len:usize) -> bool = |el, count, _, _, _| {
                (el == b'#' && (2..=3).contains(&count)) ||
                (el == b'.' && 3 == count)
            };

            let grid = self.get_grid(false);
            Self::lights_on(steps, on_logic, grid).to_string()
        }
        fn part2(&self) -> String { 
            let steps = 100usize;
            let on_logic: for<'b> fn(u8, usize, &'b Point, col_len:usize, row_len:usize) -> bool = |el, count, point, col_len, row_len| {
                point.check(0, 0) || point.check(0, row_len) || point.check(col_len, 0) || point.check(col_len, row_len) ||
                (el == b'#' && (2..=3).contains(&count)) ||
                (el == b'.' && 3 == count)
            };
            let grid = self.get_grid(true);
            Self::lights_on(steps, on_logic, grid).to_string() 
        } 
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] 
        fn test1() {
            let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
            let day = Day18::new(input.to_string());
            assert_eq!(day.part1(), 4.to_string())
        }
        #[test] 
        fn test2() {
            let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
            let day = Day18::new(input.to_string());
            assert_eq!(day.part2(), 17.to_string())
        }
    }