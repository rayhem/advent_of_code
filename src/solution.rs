mod solution {
    pub trait Solution {
        fn part_one(&self, input: &str) -> Option<String>;
        fn part_two(&self, input: &str) -> Option<String>;

        fn run(&self, input: &str) -> [Option<String>; 2] {
            [self.part_one(input), self.part_two(input)]
        }
    }
}
