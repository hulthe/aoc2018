use std::io;
use clap::{crate_name, crate_authors, crate_description, crate_version, app_from_crate, SubCommand, Arg};
use std::io::{BufRead, Stdin};

fn stdin_lines<'a>(stdin: &'a Stdin) -> Box<(dyn Iterator<Item=String> + 'a)> {
    let iter = stdin.lock()
        .lines()
        .filter_map(|r| r.ok());
    Box::new(iter)
}

fn main() {
    let app = app_from_crate!()
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help("Note that this program takes input from STDIN.")
        .subcommand(SubCommand::with_name("day_1")
            .arg(Arg::with_name("day_1_task")
                 .required(true)
                 .possible_values(&["sum", "dup"])))
        .subcommand(SubCommand::with_name("day_2")
            .arg(Arg::with_name("day_2_task")
                 .required(true)
                 .possible_values(&["checksum", "find_box"])));

    let matches = app.get_matches();

    let stdin = io::stdin();

    if let Some(sub_matches) = matches.subcommand_matches("day_1") {
        use aoc_2018_day_1::{sum_freqs, freqs_first_dup};
        match sub_matches.value_of("day_1_task") {
            Some("sum") => println!("Result: {}", sum_freqs(stdin.lock()).unwrap()),
            Some("dup") => println!("Result: {}", freqs_first_dup(stdin.lock()).unwrap()),
            _ => eprintln!("No task was issued"),
        }
    }
    else if let Some(sub_matches) = matches.subcommand_matches("day_2") {
        use aoc_2018_day_2::{checksum, find_similar_id};
        match sub_matches.value_of("day_2_task") {
            Some("checksum") => println!("Result: {}", checksum(stdin_lines(&stdin))),
            Some("find_box") => println!("Result: {}", find_similar_id(stdin_lines(&stdin))
                                         .unwrap_or("No id found...".into())),
            _ => eprintln!("No task was issued"),
        }
    } else {
        println!("{}", matches.usage());
    }
}
