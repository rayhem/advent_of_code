pub mod cli;
pub mod solution;

pub fn execute_with_timing(
    day: i32,
    input_file: &std::path::Path,
    solution: &dyn solution::Solution,
) {
    let input = std::fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("File {} not found", input_file.display()));

    let now = std::time::Instant::now();
    println!(
        "Day {0} ({2:>7}): {1:?}",
        day,
        solution.run(input.as_str()),
        now.elapsed().as_micros()
    );
}
