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
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

const SUBCOMMAND_PUSH: &'static str = "push";
const SUBCOMMAND_POP: &'static str = "pop";
const SUBCOMMAND_PEEK: &'static str = "peek";
const SUBCOMMAND_TOP: &'static str = "top";
const SUBCOMMAND_ALL: &'static str = "all";
const SUBCOMMAND_CLEAR: &'static str = "clear";

const OPTION_REVERSE: &'static str = "reverse";
const OPTION_COUNT: &'static str = "count";
const OPTION_TASK: &'static str = "task";

const MEMOL_FILE_NAME: &'static str = ".memol.json";

fn main() -> Result<()> {
    // Option
    let reverse_option = Arg::new(OPTION_REVERSE)
        .short('r')
        .takes_value(false)
        .help("stack reverse");

    let task_option = Arg::new(OPTION_TASK).value_name("TEXT");

    let count_option = Arg::new(OPTION_COUNT)
        .short('n')
        .takes_value(true)
        .value_name("NUMBER")
        .help("display count");

    // command
    let push = Command::new(SUBCOMMAND_PUSH)
        .about("push your latest task")
        .args(&[reverse_option.clone(), task_option.clone()]);

    let pop = Command::new(SUBCOMMAND_POP)
        .about("pop your latest task")
        .arg(reverse_option.clone());

    let peek = Command::new(SUBCOMMAND_PEEK)
        .about("check your latest task")
        .args(&[reverse_option.clone()]);

    let top = Command::new(SUBCOMMAND_TOP)
        .about("check your latest task")
        .args(&[reverse_option.clone()]);

    let all = Command::new(SUBCOMMAND_ALL)
        .about("all your latest task")
        .args(&[reverse_option.clone(), count_option.clone()]);

    let clear = Command::new(SUBCOMMAND_CLEAR).about("clear your latest task");

    let matches = Command::new("Your task stack")
        .version("0.2.1")
        .author("LeafChage (https://github.com/LeafChage)")
        .bin_name("memol")
        .subcommand_required(true)
        .subcommand(push)
        .subcommand(pop)
        .subcommand(peek)
        .subcommand(top)
        .subcommand(all)
        .subcommand(clear)
        .get_matches();

    let path = dirs::home_dir()
        .unwrap()
        .as_path()
        .join(Path::new(MEMOL_FILE_NAME));
    let json = JsonFile::new(&path);

    match matches.subcommand() {
        Some((SUBCOMMAND_PUSH, sub_matches)) => {
            let r = sub_matches.is_present(OPTION_REVERSE);
            let tasks = json.read()?;

            if let Some(t) = sub_matches.value_of(OPTION_TASK) {
                let task = Task::new(t, Utc::now().timestamp_millis());

                let mut tasks = if r { tasks.r() } else { tasks };
                tasks.push(task.clone());
                let tasks = if r { tasks.r() } else { tasks };
                json.sync(&tasks)?;

                println!("push {}", task);
            }
        }
        Some((SUBCOMMAND_POP, sub_matches)) => {
            let r = sub_matches.is_present(OPTION_REVERSE);
            let tasks = json.read()?;

            let mut tasks = if r { tasks.r() } else { tasks };
            let poped_task = tasks.pop();
            let tasks = if r { tasks.r() } else { tasks };
            json.sync(&tasks)?;

            if let Some(t) = poped_task {
                println!("pop {}", t);
            }
        }
        Some((SUBCOMMAND_PEEK, sub_matches)) => {
            let r = sub_matches.is_present(OPTION_REVERSE);
            let tasks = json.read()?;
            let tasks = if r { tasks.r() } else { tasks };

            if let Some(t) = tasks.peek() {
                println!("{}", t);
            }
        }
        Some((SUBCOMMAND_TOP, sub_matches)) => {
            let r = sub_matches.is_present(OPTION_REVERSE);
            let tasks = json.read()?;
            let tasks = if r { tasks.r() } else { tasks };

            if let Some(t) = tasks.peek() {
                println!("{}", t);
            }
        }
        Some((SUBCOMMAND_ALL, sub_matches)) => {
            let r = sub_matches.is_present(OPTION_REVERSE);
            let tasks = json.read()?;
            let tasks = if r { tasks.r() } else { tasks };

            let tasks = if let Some(n) = sub_matches.value_of(OPTION_COUNT) {
                if let Ok(n) = n.parse::<usize>() {
                    tasks.all_count(n)
                } else {
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
            } else {
                tasks.all()
            };

            for t in tasks.iter().rev() {
                println!("{}", t);
            }
        }
        Some((SUBCOMMAND_CLEAR, _)) => {
            let mut tasks = json.read()?;
            tasks.clear();
            json.sync(&tasks)?;
        }
        _ => unreachable!("Can I help you? ` memol -h `"),
    }

    Ok(())
}
