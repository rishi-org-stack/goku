pub mod cli;
pub mod engine;
pub mod goku;
pub mod goku_data_io;
use chrono::Local;
use cli::ExecutionCommand;
use goku::{Attack, AttackResult, ExecutionSet};
use goku_data_io::data_io::{self, Call, GokuIO};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    io::{self, Write},
    sync::mpsc::Receiver,
};

fn run_help() {
    println!("help func");
}
fn main() {
    let args: Vec<OsString> = env::args().map(OsString::from).collect();
    if args.len() < 3 {
        run_help();
        return;
    }

    let execution_type: &str = match args[1].to_str() {
        Some(first_arg) => first_arg,
        None => {
            println!("no arg found");
            return;
        }
    };

    match execution_type {
        "attack" => {
            let url = match args[2].to_str() {
                Some(u) => u,
                None => {
                    println!("url is required");
                    return;
                }
            };

            let attack_args: Vec<OsString> = args[2..].to_vec();
            let execution_command: ExecutionCommand = ExecutionCommand::new(&attack_args);

            let attack = match Attack::new(&execution_command, url) {
                Ok(a) => a,

                Err(e) => {
                    println!("failed to attack on {} err: {}", url, e.to_string());
                    return;
                }
            };

            let execution_set: ExecutionSet = match attack.run_c() {
                Ok(result) => result,
                Err(e) => {
                    println!("{e}");
                    return;
                }
            };

            if let Some(test_name) = execution_command.test_name {
                let mut path: String = test_name.clone();
                path.push_str(".logs.json");
                let mut file = fs::File::create(path).unwrap();

                file.write(execution_set.to_string().as_bytes()).unwrap();
            }
        }

        "report" => {
            println!("reporting command")
        }

        _ => run_help(),
    }
}
