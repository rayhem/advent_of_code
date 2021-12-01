pub trait Solution {
    fn part_one(&self, input: &str) -> Option<String>;
    fn part_two(&self, input: &str) -> Option<String>;

    fn run(&self, input: &str) -> [Option<String>; 2] {
        [self.part_one(input), self.part_two(input)]
    }
}

pub fn execute_with_timing<P>(day: i32, input_file: &P, solution: &Box<dyn Solution>)
where
    P: std::fmt::Debug + std::convert::AsRef<std::path::Path>,
{
    let input = std::fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("File {:?} not found", input_file));

    let now = std::time::Instant::now();
    println!(
        "Day {0:02} ({2:>7}): {1:?}",
        day,
        solution.run(input.as_str()),
        now.elapsed().as_micros()
    );
}
