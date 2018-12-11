#![feature(test)]
#![feature(await_macro)]
mod config;
mod input;

use aoc_2018_day01::Day01;
use aoc_2018_day02::Day02;
use aoc_2018_day03::Day03;
use aoc_2018_day04::Day04;
use aoc_2018_day05::Day05;
use aoc_2018_day06::Day06;
use aoc_2018_day07::Day07;
use aoc_2018_day08::Day08;
use aoc_2018_day09::Day09;
use aoc_2018_day10::Day10;
use aoc_2018_day11::Day11;
use aoc_base::AoC;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use std::error::Error;
use std::sync::{mpsc::channel, Arc};
use std::thread;
use std::time::Duration;
use std::fmt::Display;

use crate::input::get_input;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest;

macro_rules! setup_days {
    ($app:ident, $d:ident) => {{
       $app.subcommand(
            SubCommand::with_name(&stringify!($d).to_lowercase())
                .arg(Arg::with_name(concat!(stringify!($d), "Task")) //FIXME: task name
                     .required(true)
                     .possible_values(&["task_a", "task_b"])))
    }};
    ($app:ident, $d:ident, $($ds:ident),+) => {{
        let tmp = setup_days!($app, $d);
        setup_days!(tmp, $($ds),*)
    }};
}

macro_rules! run_days_async {
    ($vec:ident, $mp:ident, $d:ident) => {{
    }};
    ($vec:ident, $mp:ident, $d:ident, $($ds:ident),+) => {{
        let spinner_style = ProgressStyle::default_spinner()
            //.tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .tick_chars("|/-\\ ")
            .template("{prefix:.bold.dim} {spinner} {wide_msg}");
        let pb = $mp.add(ProgressBar::new_spinner());
        pb.set_style(spinner_style);
        pb.set_prefix(&stringify!($d));
        //pb.enable_steady_tick(100);
        let handle = thread::spawn(move|| {
            let pb = Arc::new(pb);
            let (tx, rx) = channel();
            let run = || -> Result<(), Box<Error>> {
                let pb2 = pb.clone();
                thread::spawn(move|| loop {
                    if let Ok(_) = rx.try_recv() { return; }
                    thread::sleep(Duration::from_millis(75));
                    pb2.inc(1);
                });

                pb.set_message("Fetching Data...");
                let input: String = get_input(2018, stringify!($d)[3..].parse::<u8>()?)?;

                pb.set_message("Calculating A...");
                let res_a = $d::task_a(&input)?.to_string();

                pb.set_message("Calculating B...");
                let res_b = $d::task_b(&input)?.to_string();

                fn hide_long<'a>(s: &'a str) -> &'a str {
                    if msg_is_slim(s) { s } else { "(...)" }
                };

                pb.finish_with_message(
                    &format!("Result A: {:10}   B: {}",
                             hide_long(&res_a),
                             hide_long(&res_b)));
                Ok(())
            };

            if let Err(e) = run() {
                pb.finish_with_message(&format!("Error: {}", e));
            }
            tx.send(()).ok();
        });
        $vec.push(handle);
        run_days_async!($vec, $mp, $($ds),*);
    }};
}

macro_rules! run_days {
    ($matches:ident, $d:ident) => {{
        println!("{}", $matches.usage());
    }};
    ($matches:ident, all, $($ds:ident),+) => {{
            if let Some(_) = $matches.subcommand_matches("all") {
                let mp = MultiProgress::new();
                let mut handles: Vec<_> = vec![];
                run_days_async!(handles, mp, $($ds),*);
                mp.join().unwrap();
            } else {
                run_days!($matches, $($ds),*)
            }
    }};
    ($matches:ident, $d:ident, $($ds:ident),+) => {{
        if let Some(sub_matches) = $matches.subcommand_matches(&stringify!($d).to_lowercase()) {
            let input: String = get_input(2018, stringify!($d)[3..].parse::<u8>().unwrap()).unwrap();
            match sub_matches.value_of(concat!(stringify!($d), "Task")) {
                Some("task_a") => print_result($d::task_a(&input).unwrap()),
                Some("task_b") => print_result($d::task_b(&input).unwrap()),
                _ => unreachable!("No task selected"),
            }
        } else {
            run_days!($matches, $($ds),*)
        }
    }};
}

fn msg_is_slim(msg: &str) -> bool {
    msg.len() <= 10 && !msg.contains('\n')
}

fn print_result<D: Display>(res: D) {
    let s = format!("{}", res);
    if msg_is_slim(&s) {
        println!("Result: {}", s);
    } else {
        println!("Result:\n{}", s);
    }
}

fn main() {
    let app = app_from_crate!()
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help("Don't forget to set your config.toml!")
        .subcommand(SubCommand::with_name("all").about("Compute all days"));

    let app = setup_days!(app, Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09, Day10, Day11);

    let matches = app.get_matches();

    run_days!(matches, all, Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09, Day10, Day11, FIXME);
}

#[cfg(test)]
mod test {
    extern crate test;
    use self::test::Bencher;
    use super::*;

    macro_rules! gen_bench {
        ($fna:ident, $fnb:ident, $d:ident) => {
            #[bench]
            fn $fna(b: &mut Bencher) {
                let input = get_input(2018, stringify!($d)[3..].parse::<u8>().unwrap()).unwrap();
                b.iter(|| $d::task_a(&input));
            }

            #[bench]
            fn $fnb(b: &mut Bencher) {
                let input = get_input(2018, stringify!($d)[3..].parse::<u8>().unwrap()).unwrap();
                b.iter(|| $d::task_b(&input));
            }
        };
    }

    gen_bench!(bench_day01_a, bench_day01_b, Day01);
    gen_bench!(bench_day02_a, bench_day02_b, Day02);
    gen_bench!(bench_day03_a, bench_day03_b, Day03);
    gen_bench!(bench_day04_a, bench_day04_b, Day04);
    gen_bench!(bench_day05_a, bench_day05_b, Day05);
    gen_bench!(bench_day06_a, bench_day06_b, Day06);
    gen_bench!(bench_day07_a, bench_day07_b, Day07);
    gen_bench!(bench_day08_a, bench_day08_b, Day08);
    gen_bench!(bench_day09_a, bench_day09_b, Day09);
    gen_bench!(bench_day10_a, bench_day10_b, Day10);
    gen_bench!(bench_day11_a, bench_day11_b, Day11);
}
