use super::*;

#[allow(dead_code)]
pub struct Day12 {
    input: String,
}

impl Day12 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn sum_numbers(&self) -> i32 {
        let mut total_sum = 0;
        let mut num_to_parsed = "".to_string();

        for ch in self.input.chars() {
            match (ch, &mut num_to_parsed) {
                (c, _) if c.is_ascii_digit() || c == '-' =>  {
                    num_to_parsed += c.to_string().as_str();
                }, 
                (_, num) if !num.is_empty() && num != "-" => {
                    total_sum += num.parse::<i32>().unwrap();
                    (*num).clear();
                },
                (_, _) => (),
            }
        }
        total_sum
    }

    fn sum(value: &serde_json::Value) -> i32 {
        match value {
            serde_json::Value::Number(num) => num.as_i64().unwrap() as i32,
            serde_json::Value::Array(arr) => arr.iter().map(Self::sum).sum(), 
            serde_json::Value::Object(obj) => {
                let mut sum = 0; 
                for value in obj.values() {
                    match value.as_str() {
                        Some("red") => return 0,
                        _ => sum += Self::sum(value),
                    };
                }
                sum
            },
            _ => 0, 
        }

    }

}

impl Solution for Day12 {
    fn part1(&self) -> String {
        let total = self.sum_numbers();
        format!("{}", total)
    }
    fn part2(&self) -> String {
        let json: serde_json::Value = serde_json::from_str(self.input.as_str()).unwrap();

        let total = Self::sum(&json);
        format!("{}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = "[1,2,3]{\"a\":2,\"b\":4}".to_string();
        let day = Day12::new(input);

        assert_eq!(day.part1(), "12".to_string());
    }
    #[test]
    fn test2() {
        let input = "[1,\"red\",5]{\"a\":2,\"b\":4}{\"a\":{\"b\":4},\"c\":-1}[[[3]]]{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".to_string();
        let day = Day12::new(input);
        assert_eq!(day.part2(), "18".to_string());
    }

    #[test]
    fn test3() {
        let input = std::fs::read_to_string("./src/solutions/test.txt").unwrap();
        let day = Day12::new(input);
        assert_eq!(day.part2(), "18".to_string());
    }

}
