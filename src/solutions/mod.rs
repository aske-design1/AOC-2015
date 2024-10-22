mod day1;

trait Solution<'a, T> {
    fn new(input: &'a str) -> Self;
    fn part1() -> T; 
    fn part2() -> T;
}