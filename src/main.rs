use advent::*;

fn main() {
    let cli = utils::cli::make_cli();

    let all_solutions = solutions::all_solutions();

    let root_dir = cli.value_of("input").unwrap();

    for year in utils::cli::get_cli_int_sequence("years", &cli).into_iter() {
        if let Some(year_association) = all_solutions.get(&year) {
            for day in utils::cli::get_cli_int_sequence("days", &cli).into_iter() {
                if let Some(solution) = year_association.get(&day) {
                    let fname = format!("{}/{}/day{:02}.txt", root_dir, year, day);
                    if let Ok(input) = std::fs::read_to_string(&fname) {
                        solution.execute_with_timing(day, &input);
                    } else {
                        eprintln!("File {} not found", fname);
                    }
                }
            }
        }
    }
}
