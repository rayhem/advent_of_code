use advent::{
    solutions::{year_2015, year_2020, year_2021, year_2022},
    *,
};

fn main() {
    let cli = utils::cli::make_cli();

    let y2015 = year_2015::solutions();

    let root_dir = cli.value_of("inputs").unwrap();
    for day in utils::cli::get_cli_days(&cli).into_iter() {
        if let Some(solution) = y2015.get(&day) {
            let fname = format!("{}/day{:02}.txt", root_dir, day);
            if let Ok(input) = std::fs::read_to_string(&fname) {
                solution.execute_with_timing(day, &input);
            } else {
                eprintln!("File {} not found", fname);
            }
        }
    }
}
