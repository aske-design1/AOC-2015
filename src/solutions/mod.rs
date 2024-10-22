pub mod day1;

pub trait Solution<'a> {
    fn part1(&self) -> &'a str; 
    fn part2(&self) -> &'a str;
}