use std::collections::HashMap;

use super::*;

pub struct Day7 {
    input: Vec<String>
}

struct Gate<'a> {
    operation: fn((Option<u16>, Option<u16>)) -> u16,
    values: (Option<u16>, Option<u16>),
    key: &'a str
}

impl<'a> Gate<'a> {
    fn new(operation: &Operation, key: &'a str ) -> Self {
        let operation: fn((Option<u16>, Option<u16>)) -> u16 = match operation {
            //Closures that take elements depending on its bit-operation
            Operation::And => |(a, b)| a.zip(b).map(|(a,b)| a & b).unwrap_or_else(|| panic!("Missing operand for AND")),
            Operation::Or => |(a, b)| a.zip(b).map(|(a,b)| a | b).unwrap_or_else(|| panic!("Missing operand for OR")),
            Operation::LSHIFT => |(a, b)| a.zip(b).map(|(a, b)| a << b).unwrap_or_else(|| panic!("Missing operand for LSHIFT")),
            Operation::RSHIFT => |(a, b)| a.zip(b).map(|(a, b)| a >> b).unwrap_or_else(|| panic!("Missing operand for RSHIFT")),
            Operation::Not => |(a, _)| a.map(|val| !val).unwrap_or_else(|| panic!("Missing operand for NOT")),
            Operation::Assign => |(a, _)| a.unwrap_or_else(|| panic!("Missing operand to unwrap")),
        }; 
        
        Self {
            operation,
            values: (None, None), 
            key
        }
    }

    fn execute_op(&self) -> u16 {
        (self.operation)(self.values)
    }

    fn update_values(&mut self, wires: &HashMap<&'a str, u16>, ops: &str, operation: &Operation) {
        match operation {
            Operation::Not => {
                let val = ops.split("NOT ").collect::<Vec<_>>()[1];
                self.values = match wires.get(val) {
                    Some(var) => (Some(*var), None),
                    None => return,
                };

            },
            Operation::Assign => {
                //if a variable is given and not a number, this is handled here
                self.values = match wires.get(&ops) {
                    None if ops.chars().any(|ch| ch.is_digit(10)) => {
                        (Some(ops.trim().parse::<u16>().unwrap()), None)
                    },
                    None => return,
                    Some(var) => (Some(*var), None),
                };

            }, 
            Operation::LSHIFT | Operation::RSHIFT => {
                let variable: Vec<&str> = ops.split(operation.convert_to_str()).collect();
                self.values = 
                match (wires.get(&variable[0]), variable[1].parse()) {
                    (Some(var), Ok(var2)) => (Some(*var), Some(var2)),
                    _ => return,
                };

            }
            Operation::And | Operation::Or => {
                let (val1, val2) = ops.split_once(operation.convert_to_str()).unwrap_or_else(|| panic!("Error Splitting"));
                self.values = 
                match (wires.get(val1), wires.get(val2)) {
                    (Some(var), Some(var2)) => (Some(*var), Some(*var2)),
                    (None, Some(var)) if val1.chars().any(|ch| ch.is_digit(10)) => {
                        (Some(val1.trim().parse::<u16>().unwrap()), Some(*var))
                    },
                    _ => return,
                };

                
            },
        }
    }
}

enum Operation {
    Not,
    And,
    Assign,
    Or, 
    LSHIFT,
    RSHIFT
}

impl Operation {
    fn convert(op: &str) -> Self {
        use Operation::*;
        match op {
            " AND " => And,
            " OR " => Or,
            " LSHIFT " => LSHIFT, 
            " RSHIFT " => RSHIFT, 
            "NOT " => Not,
            " -> " => Assign, 
            _ => panic!("Invalid Input")
        }
    }

    fn convert_to_str<'a>(&'a self) -> &'a str {
        match self {
            Operation::Not => "NOT ",
            Operation::And => " AND ",
            Operation::Assign => " -> ",
            Operation::Or => " OR ",
            Operation::LSHIFT => " LSHIFT ",
            Operation::RSHIFT => " RSHIFT ",
        }
    }

}

impl Day7 {
    pub fn new(input: String) -> Self {
        Self { input: input.split("\r\n").map(|line| line.to_string()).collect() }
    }

    fn operate_on_line<'a>(line: &'a str, wires: &mut HashMap<&'a str, u16>, var: Option<&str>) {
        let operation =  [" AND ", " OR ", " LSHIFT ", " RSHIFT ", "NOT ", " -> "]
        .into_iter()
        .find(|op| line.contains(*op))
        .map(Operation::convert).unwrap_or_else(||panic!("Invalid input"));
        let (ops, key) = line.split_once(" -> ").unwrap();
        let mut gate = Gate::new(&operation, key);

        //For part2
        if var.is_some_and(|val| val == gate.key) { return }


        gate.update_values(&wires, ops, &operation);
        wires.insert(gate.key, gate.execute_op());
    }
}

impl Solution for Day7 {
    fn part1(&self) -> String { 
        let mut variables: HashMap<&str, u16> = HashMap::new();

        let mut input = self.input.clone();
        input.sort_by(|a, b| {
            match a.len().cmp(&b.len()) {
                std::cmp::Ordering::Equal => a.cmp(b),
                cmp => cmp
            }
        });

        for i in 0.. {
            Self::operate_on_line(&input[i % input.len()], &mut variables, None);
            if variables.contains_key(&"a") { break; }
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

        for i in 0.. {
            Self::operate_on_line(&input[i % input.len()], &mut variables, None);
            if variables.contains_key(&"a") { break; }
        }

        let b = *variables.get(&"a").unwrap();
        variables.clear();
        variables.insert("b", b);

        for i in 0.. {
            Self::operate_on_line(&input[i % input.len()], &mut variables, Some("b"));
            if variables.contains_key(&"a") { break; }
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