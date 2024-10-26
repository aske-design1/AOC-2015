use std::collections::HashMap;

use super::*;

pub struct Day7 {
    input: Vec<String>
}

struct Gate<'a> {
    operation: fn(Option<u16>, Option<u16>) -> u16,
    first_var: Option<u16>,
    sec_var: Option<u16>,
    key: &'a str
}

impl<'a> Gate<'a> {
    fn new(operation: fn(Option<u16>, Option<u16>) -> u16, key: &'a str ) -> Self {
        Self {
            operation,
            first_var:None, 
            sec_var: None, 
            key
        }
    }
}

impl Day7 {
    pub fn new(input: String) -> Self {
        Self { input: input.split("\r\n").map(|line| line.to_string()).collect() }
    }

    fn parse_line<'a>(line: &'a str, wires: &mut HashMap<&'a str, u16>, var: Option<&str>) {
        let operations = [" AND ", " OR ", " LSHIFT ", " RSHIFT ", "NOT ", " -> "];
        let operation = operations.into_iter().find(|op| line.contains(*op)).unwrap_or("");
        
        let ops_and_res: Vec<&str> = line.split(" -> ").collect();
        let ops = ops_and_res[0];
        let gate = Gate::new(
            match operation {
            " AND " => |a, b| a.unwrap() & b.unwrap(),
            " OR " => |a, b| a.unwrap() | b.unwrap(),
            " LSHIFT " => |a, b| a.unwrap() << b.unwrap(),
            " RSHIFT " => |a, b| a.unwrap() >> b.unwrap(),
            "NOT " => |a, _| !a.unwrap(),
            " -> " => |a, _| a.unwrap(),
            _ => panic!("Invalid input"),
            }, 
            ops_and_res[1]
        );

        //For part2
        if let Some(unpacked_var) = var {
            if unpacked_var == gate.key { return }
        }

        Self::operate(wires, ops, operation, gate);
    }

    fn operate<'a>(wires: &mut HashMap<&'a str, u16>, ops: &str, operation: &str, mut gate: Gate<'a>) {
        match operation {
            "NOT " => {
                let var: Vec<&str> = ops.split("NOT ").collect();
                gate.first_var = match wires.get(var[1]) {
                    Some(var) => Some(*var),
                    None => return,
                };

                let res = (gate.operation)(gate.first_var, None);
                wires.insert(gate.key, res);
            },
            " -> " => {
                //if a variable is given and not a number, this is handled here
                gate.first_var = match wires.get(&ops) {
                    None if ops.chars().any(|ch| ch.is_digit(10)) => {
                        Some(ops.trim().parse::<u16>().unwrap()) 
                    },
                    None => return,
                    Some(var) => Some(*var),
                };

                let res = (gate.operation)(gate.first_var, None);
                wires.insert(gate.key, res);
            }, 
            " LSHIFT " | " RSHIFT " => {
                let variable: Vec<&str> = ops.split(operation).collect();
                (gate.first_var, gate.sec_var) = 
                match (wires.get(&variable[0]), variable[1].parse()) {
                    (Some(var), Ok(var2)) => (Some(*var), Some(var2)),
                    _ => return,
                };

                let res = (gate.operation)(gate.first_var, gate.sec_var);
                wires.insert(gate.key, res);
            }
            " AND " | " OR " => {
                let variable: Vec<&str> = ops.split(operation).collect();

                (gate.first_var, gate.sec_var) = 
                match (wires.get(&variable[0]), wires.get(&variable[1])) {
                    (Some(var), Some(var2)) => (Some(*var), Some(*var2)),
                    (None, Some(var)) if variable[0].chars().any(|ch| ch.is_digit(10)) => {
                        (Some(variable[0].trim().parse::<u16>().unwrap()), Some(*var))
                    },
                    _ => return,
                };

                let res = (gate.operation)(gate.first_var, gate.sec_var);
                wires.insert(gate.key, res);
            },
            _ => panic!("Invalid")
        }
    }

}

impl Solution for Day7 {
    type Output = String;
    fn part1(&self) -> String { 
        let mut variables: HashMap<&str, u16> = HashMap::new();

        let mut input = self.input.clone();
        
        input.sort_by(|a, b| {
            let cmp = a.len().cmp(&b.len());
            if cmp == std::cmp::Ordering::Equal {
                a.cmp(b)
            } else {
                cmp
            }
        });

        let mut i = 0; 
        while !variables.contains_key(&"a") {
            Self::parse_line(&input[i % input.len()], &mut variables, None);
            i += 1;
        }

        format!("{}", 
            match variables.get(&"a")  {
                Some(var) => *var, 
                None => 0
            }
        ) 
    }
    
    fn part2(&self) -> String { 
        let mut variables: HashMap<&str, u16> = HashMap::new();
        let input = self.input.clone();

        let mut i = 0; 
        while !variables.contains_key(&"a") {
            Self::parse_line(&input[i % input.len()], &mut variables, None);
            i += 1;
        }

        let b = *variables.get(&"a").unwrap();
        variables.clear();
        variables.insert("b", b);
        i = 0;
        while !variables.contains_key(&"a") {
            Self::parse_line(&input[i % input.len()], &mut variables, Some("b"));
            i += 1;
        }

        format!("{}", 
            match variables.get(&"a")  {
                Some(var) => *var, 
                None => 0
            }
        ) 
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test1() {
        let input = "123 -> x\r\n456 -> y\r\nx AND y -> d\r\nx OR y -> e\r\nx LSHIFT 2 -> f\r\ny RSHIFT 2 -> g\r\nNOT x -> h\r\nNOT y -> i\r\n1 AND x -> a";
        let day = Day7::new(input.to_string());
        assert_eq!(day.part1(), "1")
    }
    #[test] fn test2() {}
}