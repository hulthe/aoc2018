use std::io;
use clap::{crate_name, crate_authors, crate_description, crate_version, app_from_crate, SubCommand, Arg};


fn main() {
    let app = app_from_crate!()
        .subcommand(SubCommand::with_name("day_1")
            .about("Solve day 1")
            .after_help("Note that this module takes input from STDIN.")
            .version("1.0")
            .arg(Arg::with_name("day_1_task")
                 .required(true)
                 .possible_values(&["sum", "dup"])));
    let matches = app.get_matches();

    let stdin = io::stdin();

    if let Some(sub_matches) = matches.subcommand_matches("day_1") {
        use aoc_2018_day_1::{sum_freqs, freqs_first_dup};
        match sub_matches.value_of("day_1_task") {
            Some("sum") => println!("Result: {}", sum_freqs(stdin.lock()).unwrap()),
            Some("dup") => println!("Result: {}", freqs_first_dup(stdin.lock()).unwrap()),
            _ => eprintln!("No task was issued"),
        }
    } else {
        println!("{}", matches.usage());
    }
}
