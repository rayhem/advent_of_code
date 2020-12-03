pub mod day01;
pub mod day02;

pub trait Solution {
    fn part_one(&self, input: &str) -> Option<String>;
    fn part_two(&self, input: &str) -> Option<String>;

    fn run(&self, input: &str) -> Vec<Option<String>> {
        vec![self.part_one(input), self.part_two(input)]
    }
}
