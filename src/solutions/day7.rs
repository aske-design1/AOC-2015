use std::collections::HashMap;

use super::*;

#[allow(dead_code)]
pub struct Day7 {
    input: Vec<String>
}

impl Day7 {
    pub fn new(input: String) -> Self {
        Self { input: input.split("\r\n").map(|line| line.to_string()).collect() }
    }
    fn parse_line<'a>(line: &'a str, variables: &mut HashMap<&'a str, u16>) {
        let operations = [" AND ", " OR ", " LSHIFT ", " RSHIFT ", "NOT ", " -> "];
        let &operation = operations.iter().find(|&&op| line.contains(op)).unwrap_or(&"");

        let op: fn(u16, Option<u16>) -> u16 = match operation {
            " AND " => |a, b| a & b.unwrap(),
            " OR " => |a, b| a | b.unwrap(),
            " LSHIFT " => |a, b| a << b.unwrap(),
            " RSHIFT " => |a, b| a >> b.unwrap(),
            "NOT " => |a, _| !a,
            " -> " => |a, _| a,
            _ => panic!("Invalid input"),
        };
        //println!("{}", line);

        if operation == "NOT " || operation == " -> " {
            let key: Vec<&str> = line.split(" -> ").collect();

            let value = match operation {
                "NOT " => {
                    //Need to be split further
                    let var: Vec<&str> = key[0].split("NOT ").collect();
                    println!("_{}_and_{}_", var[0], var[1]);

                    *variables.get(var[1]).expect("Variable not found")
                },
                " -> " => key[0].trim().parse().expect("Failed to parse value"),
                _ => unreachable!(),
            };

            let res = op(value, None);
            variables.insert(key[1], res);
            return;
        }

        let variable: Vec<&str> = line.split(operation).collect();
        let var2_and_key: Vec<&str> = variable[1].split(" -> ").collect();
        
        

        //If lshift or rshift then sec_val is a num
        let sec_val = match operation {
            " LSHIFT " | " RSHIFT " => var2_and_key[0].parse().unwrap(),
            _ => *variables.get(&var2_and_key[0]).unwrap()
        };


        let res = op(*variables.get(&variable[0]).unwrap(), 
        Some(sec_val));
        variables.insert(var2_and_key[1], res);
    }

}

impl Solution for Day7 {
    fn part1(&self) -> String { 
        let mut variables: HashMap<&str, u16> = HashMap::new();

        let mut input = self.input.clone();
        
        todo!("Sort by length, then by lexiconal val");
        input.sort();

        for line in input.iter() {
            let instruct = Self::parse_line(line, &mut variables);
        }
        
        format!("{}", 
            match variables.get(&"a")  {
                Some(var) => *var, 
                None => 0
            }
        ) 
    }
    fn part2(&self) -> String { format!("") } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test1() {
        let input = "123 -> x\r\n456 -> y\r\nx AND y -> d\r\nx OR y -> e\r\nx LSHIFT 2 -> f\r\ny RSHIFT 2 -> g\r\nNOT x -> h\r\nNOT y -> i";
        let day = Day7::new(input.to_string());
        assert_eq!(day.part1(), "0")
    }
    #[test] fn test2() {}
}