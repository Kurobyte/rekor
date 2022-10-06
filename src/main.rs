mod cli;
mod git;
mod track;

// std
use std::{env, process::Child};
use std::io::Error;
use std::process::Command;
use std::collections::VecDeque;
use std::sync::mpsc::channel;

// external modules
use chrono::{Local, Duration};
use clap::Parser;
use regex::Regex;
use which::which;
use ctrlc;

// own modules
use cli::CliArgs;
use git::git_utils;
use track::tfile;

fn diff_time(diff: Duration) -> String {
    format!("{0:02}:{1:02}:{2:02}", diff.num_hours(), diff.num_minutes() % 60, diff.num_seconds() % 60)
}

fn parse_cli_args() -> VecDeque<String> {
    let mut args = VecDeque::<String>::from_iter(env::args().into_iter());
    args.pop_front();

    args
}

fn execute_subcommand(cmd: &String, args: VecDeque<String>) -> Result<Child, Error> {
    Command::new(cmd).args(args).env("PATH", env::var("PATH").unwrap()).spawn()
}

fn main() {
    let clarg: CliArgs;
    let reg = Regex::new(r"^-").unwrap();
    let (tx, rx) = channel();
    let txc = tx.clone();
    let mut args: VecDeque<String> = parse_cli_args();

    let _ = ctrlc::set_handler(move || {
        let _ = txc.send(false);
    });

    if args.is_empty() {
        println!("No s'han passat arguments.");
        return;
    }
    let cmd = args.pop_front().unwrap();

    if reg.is_match(cmd.as_str()) {
        println!("Comanda especial.");
        println!("{:?}", args);
        clarg = CliArgs::parse();
        println!("{:?}", clarg);
    } else {
        let project;
        let desc_branch;

        let cmd_path_res = which(&cmd);
        let cmd_path;

        if cmd_path_res.is_err() {
            println!("Commanda no trobada");
            return;
        }
        
        cmd_path = cmd_path_res.unwrap();

        if git_utils::is_git_dir() {
            project = git_utils::get_project_name();
            desc_branch = git_utils::get_branch_name();
        } else {
            project = "NO_PROJECT".to_owned();
            desc_branch= "NO_BRANCH".to_owned();
        }

        let start = Local::now();
        let exec_result = execute_subcommand(&cmd_path.as_os_str().to_str().unwrap().to_string(), args);
        if exec_result.is_err() {
            // dbg!(env::var("PATH").unwrap());
            // dbg!(find_it("mitra"));
            // let mut wh_args: VecDeque<String> = VecDeque::new();
            // wh_args.push_front(&cmd);
            // dbg!(execute_subcommand(&"where".to_string(), wh_args));
            // dbg!(which(&cmd));
            dbg!(exec_result.err());
            return; // Bloquejem la execució
        }

        let mut child = exec_result.unwrap();
        let res_child = child.wait();
        match res_child {
            Ok(_) => { let _ = tx.send(true); },
            Err(_) => todo!(),
        }

        let _rx_data = rx.recv().unwrap();
        // if rx_data {
        //     println!("Execució OK");
        // } else {
        //     println!("Execució forçada");
        // }
        
        let end = Local::now();
        let diff = end - start;
        //csv:
        //project;mes;dia;hora_inici;min_inici;duracio;descripcio
        tfile::write_raw(
            format!("{};{};{};{};{};{}", project, start.format("%m"), start.format("%d"), start.format("%H:%M"), diff_time(diff), desc_branch)
        );
    }
}
