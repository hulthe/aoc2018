use std::io;
use clap::{crate_name, crate_authors, crate_description, crate_version, app_from_crate, SubCommand, Arg};
use std::io::{BufRead, Stdin};
use aoc_base::AoC;

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
                 .possible_values(&["checksum", "find_box"])))
        .subcommand(SubCommand::with_name("day_3")
            .arg(Arg::with_name("day_3_task")
                 .required(true)
                 .possible_values(&["overlapping", "safe_claims"])))
        .subcommand(SubCommand::with_name("day_4")
            .arg(Arg::with_name("day_4_task")
                 .required(true)
                 .possible_values(&["task_a", "task_b"])));

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
    }
    else if let Some(sub_matches) = matches.subcommand_matches("day_3") {
        use aoc_2018_day_3::{overlapping, safe_claim};
        match sub_matches.value_of("day_3_task") {
            Some("overlapping") => println!(
                "Result: {}",
                overlapping(stdin_lines(&stdin)).unwrap()),
            Some("safe_claims") => println!(
                "Result: {}",
                safe_claim(stdin_lines(&stdin)).unwrap()),
            v => eprintln!("Not a valid task: {:?}", v),
        }
    }
    else if let Some(sub_matches) = matches.subcommand_matches("day_4") {
        use aoc_2018_day_4::{Day4};
        match sub_matches.value_of("day_4_task") {
            Some("task_a") => println!(
                "Result: {}",
                Day4::task_a(stdin_lines(&stdin)).unwrap()),
            Some("task_b") => println!(
                "Result: {}",
                Day4::task_b(stdin_lines(&stdin)).unwrap()),
            v => eprintln!("Not a valid task: {:?}", v),
        }
    } else {
        println!("{}", matches.usage());
    }
}
