extern crate chrono;
extern crate clap;
extern crate serde;
extern crate serde_json;

mod stack;

use clap::{Arg, Command};
use stack::json::{JsonFile, Task};

use chrono::prelude::*;
use std::io::Result;

const SUBCOMMAND_PUSH: &'static str = "push";
const SUBCOMMAND_POP: &'static str = "pop";
const SUBCOMMAND_PEEK: &'static str = "peek";
const SUBCOMMAND_TOP: &'static str = "top";

const MEMO_FILE_PATH: &'static str = "~/.memol.json";

fn main() -> Result<()> {
    let mut json = JsonFile::init(MEMO_FILE_PATH)?;

    let matches = Command::new("memol")
        .bin_name("cargo")
        .subcommand_required(true)
        .subcommand(
            Command::new(SUBCOMMAND_PUSH)
                .about("push your latest task")
                .arg(Arg::new("task")),
        )
        .subcommand(Command::new(SUBCOMMAND_POP).about("pop your latest task"))
        .subcommand(Command::new(SUBCOMMAND_PEEK).about("check your latest task"))
        .subcommand(Command::new(SUBCOMMAND_TOP).about("check your latest task"))
        .get_matches();

    match matches.subcommand() {
        Some((SUBCOMMAND_PUSH, sub_matches)) => {
            if let Some(t) = sub_matches.value_of("task") {
                json.tasks()
                    .push(Task::new(t, Utc::now().timestamp_millis()));
            }
        }
        Some((SUBCOMMAND_POP, _)) => {
            json.tasks().pop();
        }
        Some((SUBCOMMAND_PEEK, _)) => {
            json.tasks().peek();
        }
        Some((SUBCOMMAND_TOP, _)) => {
            json.tasks().top();
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    json.sync()?;
    Ok(())
}
