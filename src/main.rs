use advent::*;

fn main() {
    let cli = utils::cli::CommandLineInterface::new();
    let root_dir = cli.get_input_dir();

    let all_solutions = solutions::all_solutions();

    for year in cli.get_years() {
        if let Some(year_association) = all_solutions.get(&year) {
            for day in cli.get_days() {
                if let Some(solution) = year_association.get(&day) {
                    let fname = root_dir
                        .join(year.to_string())
                        .join(format!("day{0:02}", day))
                        .with_extension("txt");
                    if let Ok(input) = std::fs::read_to_string(&fname) {
                        solution.execute_with_timing(day, &input);
                    } else {
                        eprintln!("File {} not found", fname.display());
                    }
                }
            }
        }
    }
}
