extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate serde;
extern crate serde_json;

mod stack;

use chrono::prelude::*;
use clap::{Arg, Command};
use stack::json::JsonFile;
use stack::task::Task;
use std::io::Result;
use std::path::Path;

const SUBCOMMAND_PUSH: &'static str = "push";
const SUBCOMMAND_POP: &'static str = "pop";
const SUBCOMMAND_PEEK: &'static str = "peek";
const SUBCOMMAND_TOP: &'static str = "top";

const MEMOL_FILE_NAME: &'static str = ".memol.json";

fn main() -> Result<()> {
    let path = dirs::home_dir()
        .unwrap()
        .as_path()
        .join(Path::new(MEMOL_FILE_NAME));

    let mut json = JsonFile::init(&path)?;

    let matches = Command::new("Your task stack")
        .version("0.1.0")
        .author("LeafChage (https://github.com/LeafChage)")
        .bin_name("memol")
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

                if let Some(t) = json.tasks().peek() {
                    println!("push {}", t);
                }
            }
        }
        Some((SUBCOMMAND_POP, _)) => {
            json.tasks().pop();
            if let Some(t) = json.tasks().peek() {
                println!("pop {}", t);
            }
        }
        Some((SUBCOMMAND_PEEK, _)) => {
            if let Some(t) = json.tasks().peek() {
                println!("{}", t);
            }
        }
        Some((SUBCOMMAND_TOP, _)) => {
            if let Some(t) = json.tasks().top() {
                println!("{}", t);
            }
        }
        _ => unreachable!("Can I help you? ` memol -h `"),
    }

    json.sync()?;
    Ok(())
}
