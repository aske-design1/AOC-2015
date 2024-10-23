use super::Solution;

pub struct Day6 {
    input: Vec<String>,
}

struct Op<'a> {
    cmd: &'a str,
    row: (usize, usize),
    col: (usize, usize),
}

impl Day6 {
    pub fn new(input: String) -> Self {
        Self {
            input: input.split("\r\n").map(|line| line.to_string()).collect(),
        }
    }

    fn extract_instructions<'a>(line: &'a str) -> Op<'a> {
        let (cmd, rest) = line.split_at(
            line.find(|c: char| c.is_ascii_digit())
                .unwrap_or(line.len())
        );        

        let cmd = cmd.trim();
        let mut rest = rest.split_whitespace();
        let from = rest.next().unwrap();
        rest.next();
        let to = rest.next().unwrap();

        let from_coords: Vec<usize> = from.split(',').map(|x| x.parse().unwrap()).collect();
        let to_coords: Vec<usize> = to.split(',').map(|x| x.parse().unwrap()).collect();   
        
        Op {
            cmd,
            row: (from_coords[0], to_coords[0]),
            col: (from_coords[1], to_coords[1])
        }
    }

    fn light_operations(line: &str, lights: &mut [[u8; 1000]; 1000]) {
        //println!("{}", line);
        let instructs = Self::extract_instructions(line);

        let op: fn(u8) -> u8  = match instructs.cmd {
            "turn on" => |_| 1,
            "toggle" => |a| a ^ 1,
            "turn off" => |_| 0,
            _ => panic!("Error: invalid input")
        };
        for i in instructs.row.0..=instructs.row.1 {
            for j in instructs.col.0..=instructs.col.1 {
                lights[i][j] = op(lights[i][j])
            }
        }

    }

    fn light_operations_part_2(line: &str, lights: &mut Vec<Vec<u16>>) {
        let instructs = Self::extract_instructions(line);

        let op: fn(u16) -> u16  = match instructs.cmd {
            "turn on" => |a| a + 1,
            "toggle" => |a| a + 2,
            "turn off" => |a| if a != 0 { a - 1 } else { a },
            _ => panic!("Error: invalid input")
        };
        for i in instructs.row.0..=instructs.row.1 {
            for j in instructs.col.0..=instructs.col.1 {
                lights[i][j] = op(lights[i][j])
            }
        }

    }
}

impl Solution for Day6 {
    fn part1(&self) -> String {
        let mut light_arr = [[0u8; 1000]; 1000];

        for line in self.input.iter() {
            Self::light_operations(line, &mut light_arr);
        }

        format!(
            "{}",
            light_arr.iter().fold(0, |acu, arr| acu
                + arr.iter().fold(0, |acc, el| acc + (*el as u32)))
        )
    }
    fn part2(&self) -> String {
        let mut light_arr = vec![vec![0u16; 1000]; 1000];

        for line in self.input.iter() {
            Self::light_operations_part_2(line, &mut light_arr);
        }

        format!(
            "{}",
            light_arr.iter().fold(0, |acu, arr| acu
                + arr.iter().fold(0, |acc, el| acc + (*el as u32)))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "turn on 0,0 through 999,999\r\ntoggle 0,0 through 999,0\r\nturn off 499,499 through 500,500";

        let test = Day6::new(input.to_string());
        assert_eq!(test.part1(), "998996".to_string());
    }
    #[test]
    fn test2() {
        let input = "toggle 0,0 through 999,999\r\nturn off 0,0 through 0,0";

        let test = Day6::new(input.to_string());
        assert_eq!(test.part2(), "1999999".to_string());
    }
}
