use super::*;

    #[derive(Clone)]
    struct Reindeer {
        speed: u64,
        active: u64,
        rest_time: u64
    }
    impl Reindeer {
        fn new(speed: u64, active: u64, rest_time: u64) -> Self { Self { speed, active, rest_time } }
    }

    #[allow(dead_code)]
    pub struct Day14 {
        input: Vec<String>
    }

    impl Day14 {
        pub fn new(input: String) -> Self {
            let splitter = if input.contains("\r\n") { "\r\n" } else { "\n" };
            Self { input: input.split(splitter).map(|line| line.to_string()).collect::<Vec<String>>() }
        }

        fn distance_of_reindeer(reindeer: Reindeer, amount_of_sec: u64) -> u64 {
            let cycle_length = reindeer.active + reindeer.rest_time; 
            let cycle_amount = amount_of_sec / cycle_length;
            
            //First part is distanced gained from every full cycle 
            //and last is how much of active is caught
            reindeer.speed * reindeer.active * cycle_amount + 
            reindeer.speed * std::cmp::min(amount_of_sec - (cycle_amount * cycle_length), reindeer.active)  
        }
        fn parse_line(line: &str) -> Reindeer {
            let split_line = line.split(" ").collect::<Vec<&str>>();
            Reindeer::new(
                split_line[3].parse().unwrap(), 
                split_line[6].parse().unwrap(), 
                split_line[13].parse().unwrap()
            )
        }

        fn new_rules(reindeers: Vec<Reindeer>, amount_of_sec: usize) -> u64 {
            let mut scores: Vec<u64> = vec![0; reindeers.len()];
            for i in 1..amount_of_sec {
                let sec = i as u64;
                let mut score_at_sec: Box<Vec<(u64, usize)>> = Box::new(Vec::new());

                for (idx, reindeer) in reindeers.iter().enumerate() {
                    //at .0 distance, at .1 index 
                    score_at_sec.push((Self::distance_of_reindeer(reindeer.clone(), sec), idx));
                }
                //Reversed sorting such that largest is 1st
                score_at_sec.sort_by(|a, b| b.0.cmp(&a.0));
                let winner = score_at_sec.first().unwrap(); 
                scores[winner.1] += 1; 

                let mut i = 1;
                while let Some(score) = score_at_sec.get(i) {
                    if winner.0 != score.0 { break }
                    scores[score.1] += 1; 
                    i+=1; 
                }
            }
            scores.sort();
            *scores.last().unwrap()
        }

    }

    impl Solution for Day14 {
        fn part1(&self) -> String { 
            let mut max_dist:u64 = 0;
            for line in self.input.iter() {
                let reindeer = Self::parse_line(line);
                let dist = Self::distance_of_reindeer(reindeer, 2503);
                max_dist = std::cmp::max(dist, max_dist);
            }
            max_dist.to_string() 
        }
        fn part2(&self) -> String { 
            let reindeer_vec = 
            self.input.iter().map(|line| Self::parse_line(line)).collect::<Vec<Reindeer>>();

            Self::new_rules(reindeer_vec, 2503).to_string()     
        } 
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test] 
        fn test1() {
            let input = 
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
            let day = Day14::new(input.to_string());

            assert_eq!(day.part1(), 2660.to_string());
        }
        #[test] 
        fn test2() {
            let input = 
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
            let day = Day14::new(input.to_string());

            assert_eq!(day.part2(), 1564.to_string());
        }
    }