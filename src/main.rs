extern crate chrono;
extern crate clap;
extern crate serde;
extern crate serde_json;

mod stack;

use clap::{Arg, Command};

const SUBCOMMAND_PUSH: &'static str = "push";
const SUBCOMMAND_POP: &'static str = "pop";
const SUBCOMMAND_PEEK: &'static str = "peek";
const SUBCOMMAND_TOP: &'static str = "top";

fn main() {
    let matches = Command::new("memol")
        .bin_name("cargo")
        .subcommand_required(true)
        .subcommand(
            Command::new(SUBCOMMAND_PUSH)
                .about("push your latest task")
                .arg(Arg::new("task")),
        )
        .subcommand(
            Command::new(SUBCOMMAND_POP)
                .about("pop your latest task")
                .arg(Arg::new("task")),
        )
        .subcommand(Command::new(SUBCOMMAND_PEEK).about("check your latest task"))
        .subcommand(Command::new(SUBCOMMAND_TOP).about("check your latest task"))
        .get_matches();

    match matches.subcommand() {
        Some((SUBCOMMAND_PUSH, sub_matches)) => {
            println!("{} {:?}", SUBCOMMAND_PUSH, sub_matches.value_of("task"))
        }
        Some((SUBCOMMAND_POP, sub_matches)) => {
            println!("{} {:?}", SUBCOMMAND_POP, sub_matches.value_of("task"))
        }
        Some((SUBCOMMAND_PEEK, sub_matches)) => {
            println!("{}", SUBCOMMAND_PEEK)
        }
        Some((SUBCOMMAND_TOP, sub_matches)) => {
            println!("{}", SUBCOMMAND_TOP)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
