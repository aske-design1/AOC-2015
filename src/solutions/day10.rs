
use super::*;
use itertools::Itertools;

pub struct Day10 {
    input: String
}

impl Day10 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn num_to_bytes(mut n: usize) -> impl Iterator<Item = u8> {
        std::iter::from_fn(move || 
            if n > 0 {
                let prev_n = n;
                n /= 10;
                Some(b'0' + (prev_n % 10) as u8)
            } else { None }
        ).collect::<Vec<_>>().into_iter().rev()   
    }

    fn look_and_say(&self, iterations: usize) -> String {
        String::from_utf8(
            (0..iterations).into_iter().fold(self.input.clone().as_bytes().to_vec(), |seq, _| {
                seq.iter()
                .chunk_by(|&byte| byte)
                .into_iter().map(|(byte, group) |
                    Self::num_to_bytes(group.count()).chain([*byte].into_iter()).collect::<Vec<_>>()
                ).flatten().collect()
            })
        ).unwrap()
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