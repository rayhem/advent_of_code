pub mod cli;
pub mod solution;

pub mod advent_utils {
    pub fn execute_with_timing<P: std::fmt::Debug + std::convert::AsRef<std::path::Path>>(
        day: i32,
        input_file: &P,
        solution: &dyn crate::solution::Solution,
    ) {
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
}
