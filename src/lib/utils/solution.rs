pub trait Solution {
    fn part_one(&self, input: &str) -> Option<String>;
    fn part_two(&self, input: &str) -> Option<String>;

    fn run(&self, input: &str) -> [Option<String>; 2] {
        [self.part_one(input), self.part_two(input)]
    }

    fn execute_with_timing(&self, day: i32, input: &str) {
        let now = std::time::Instant::now();
        let s = self.run(input);
        let time = now.elapsed().as_micros();

        println!("Day {:02} ({:>7}): {:?}", day, time, s);
    }
}
