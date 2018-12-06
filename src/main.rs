#![feature(await_macro)]
use aoc_2018_day1::Day1;
use aoc_2018_day2::Day2;
use aoc_2018_day3::Day3;
use aoc_2018_day4::Day4;
use aoc_2018_day5::Day5;
use aoc_2018_day6::Day6;
use aoc_base::AoC;
use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{mpsc::channel, Arc};
use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest;

#[derive(Deserialize)]
struct Config {
    url: String,
    session: String,
}

fn take_input(year: u32, day: u8) -> impl IntoIterator<Item = String> {
    let mut config_data = String::new();
    let mut file = File::open("config.toml").expect("Could not open config");
    file.read_to_string(&mut config_data)
        .expect("Could not read config");
    let config: Config = toml::from_str(&config_data).expect("Could not parse config");

    let client = reqwest::Client::new();
    let mut resp = client
        .get(&format!("{}/{}/day/{}/input", config.url, year, day))
        .header("cookie", format!("session={}", config.session))
        .send()
        .expect("Could not connect to adventofcode");
    let body = resp.text().expect("Could not get response body");
    body.lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

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
            let (tx, rx) = channel();
            let pb = Arc::new(pb);
            let pb2 = pb.clone();
            thread::spawn(move|| loop {
                if let Ok(_) = rx.try_recv() { return; }
                thread::sleep(Duration::from_millis(75));
                pb2.inc(1);
            });
            //pb.enable_steady_tick(100);

            pb.set_message("fetching data...");
            let input: Vec<_> = take_input(2018, stringify!($d)[3..].parse::<u8>().unwrap()).into_iter().collect();

            pb.set_message("calculating a...");
            let res_a = $d::task_a(input.clone()).unwrap();

            pb.set_message("calculating b...");
            let res_b = $d::task_b(input).unwrap();

            tx.send(()).ok();
            pb.finish_with_message(&format!("Result A: {:7}   B: {}", res_a, res_b));
        });
        $vec.push(handle);
        run_days_async!($vec, $mp, $($ds),*);
    }};
}

macro_rules! run_days {
    ($matches:ident, $d:ident) => {{
        unreachable!("No day selected");
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
            let input = take_input(2018, stringify!($d)[3..].parse::<u8>().unwrap());
            match sub_matches.value_of(concat!(stringify!($d), "Task")) {
                Some("task_a") => println!("Result: {}", $d::task_a(input).unwrap()),
                Some("task_b") => println!("Result: {}", $d::task_b(input).unwrap()),
                _ => unreachable!("No task selected"),
            }
        } else {
            run_days!($matches, $($ds),*)
        }
    }};
}

fn main() {
    let app = app_from_crate!()
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_help("Don't forget to set your config.toml!")
        .subcommand(SubCommand::with_name("all").about("Compute all days"));

    let app = setup_days!(app, Day1, Day2, Day3, Day4, Day5, Day6);

    let matches = app.get_matches();

    run_days!(matches, all, Day1, Day2, Day3, Day4, Day5, Day6, FIXME);
}
