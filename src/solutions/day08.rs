use super::Solution;


pub struct Day8 {
    input: Vec<Vec<u8>>,
}

impl Day8 {
    pub fn new(input: Vec<u8>) -> Self { 
        Self { input: Self::split_on_crlf(&input) }
    }
    fn split_on_crlf(input: &[u8]) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        let mut start = 0;
        for i in 1..input.len() {
            // Check for the \r\n sequence
            if input[i - 1] == b'\r' && input[i] == b'\n' {
                // Collect the bytes up to this point, excluding the \r\n
                if start < i - 1 { result.push(input[start..i - 1].to_vec()); }
                // Update the start to the byte after \n
                start = i + 1;
            }
        }
        // Add the remaining bytes after the last \r\n
        if start < input.len() { result.push(input[start..].to_vec()); }
        result
    }

    fn convert_to_string_len(line: &Vec<u8>) -> usize {
        let mut i = 1; 
        let mut total_len = 0; 
        while i < line.len() - 1 {
            (i, total_len) = match (line[i], line[i+1]) {
                (b'\\', b'x') => (i + 4, total_len + 1),
                (b'\\', _) => (i + 2, total_len + 1),
                (_, _) =>  (i + 1, total_len + 1),
            };
        }
        total_len
    }

    fn convert_to_encode_len(line: &Vec<u8>) -> usize {
        let mut total_len = 0; 
        for i in 0..line.len() {
            total_len += match line[i] {
                b'"' => 2,
                b'\\' => 2,
                _ => 1,
            }
        }
        // Add the start and end ""-marks to the total length 
        total_len + 2
    }
}

impl Solution for Day8 {
    fn part1(&self) -> String {
        let mut total_literal_len = 0;
        let mut total_string_len = 0;

        for line in self.input.iter() {
            total_string_len += Self::convert_to_string_len(line);
            total_literal_len += line.len();
        }
        
        format!("{}", total_literal_len - total_string_len)
    }
    fn part2(&self) -> String {
        let mut total_literal_len = 0;
        let mut total_encode_len = 0;

        for line in self.input.iter() {
            total_literal_len += line.len();
            total_encode_len += Self::convert_to_encode_len(line);
        }
        
        format!("{}", total_encode_len - total_literal_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //use super::*;
    #[test] fn test1() {
        let input: Vec<Vec<u8>> = vec![
            b"\"\"".to_vec(),         
            b"\"abc\"".to_vec(),        
            b"\"aaa\\\"aaa\"".to_vec(), 
            b"\"\\x27\"".to_vec()
        ];
        let day = Day8 { input };
        assert_eq!(day.part1(), "12".to_string());
    }
    #[test] fn test2() {
        let input: Vec<Vec<u8>> = vec![
            b"\"\"".to_vec(),         
            b"\"abc\"".to_vec(),        
            b"\"aaa\\\"aaa\"".to_vec(), 
            b"\"\\x27\"".to_vec()       
        ];
        let day = Day8 { input };
        assert_eq!(day.part2(), "19".to_string());
    }
}
