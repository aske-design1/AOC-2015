
use super::*;
    pub struct Day10 {
        input: String
    }

    impl Day10 {
        pub fn new(input: String) -> Self {
            Self { input }
        }

        fn look_and_say(&self, iterations: usize) -> String {
            let mut seq = self.input.clone();
            for _ in 0..iterations {
                let mut new_seq = "".to_string();

                let mut i = 0; 
                let seq_bytes = seq.as_bytes();
                while let Some(first_met_byte) = seq_bytes.get(i) {
                    
                    i+=1;
                    let mut counter = 1;
                    while let Some(val) = seq_bytes.get(i) {
                        if *first_met_byte != *val { break; }

                        counter += 1;
                        i+=1;
                    }
                    let append_str = format!("{}{}", counter, *first_met_byte as char);
                    new_seq += append_str.as_str(); 
                } 
                seq = new_seq;
            }
            seq
        }
    }

    impl Solution for Day10 {
        fn part1(&self) -> String { 
            format!("{}", self.look_and_say(40).len()) 
        }
        fn part2(&self) -> String { 
            format!("{}", format!("{}", self.look_and_say(50).len()) ) 
        } 
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] fn test1() {
            let day = Day10::new("1".to_string());
            assert_eq!(day.look_and_say(5), "312211".to_string());
        }
        #[test] fn test2() {}
    }