use std::process::{Command, Stdio, Output};
use std::io::Error;
use std::path::Path;


pub fn is_git_dir() -> bool {
    let result = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match result {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

/// Get the project name if a git is initialized.
/// If the project git hasn't been initialized or the execution panics it will return `NO_NAME`.
pub fn get_project_name() -> String {
    let empty_name = "NO_NAME".to_string();

    match execute_command_w_output(vec!["rev-parse", "--show-toplevel"]) {
        Ok(outp) => {
            return if outp.status.success() {
                let paths = String::from_utf8(outp.stdout).unwrap_or(empty_name);
                let path = Path::new(paths.as_str());
                path.file_name().unwrap().to_str().unwrap().trim().to_string()
            } else {
                empty_name
            };
        }
        Err(_) => empty_name,
    }
}

pub fn get_branch_name() -> String {
    let no_branch: &str = "NO_BRANCH";
    let result = match execute_command_w_output(vec!["branch", "--show-current"]) {
        Ok(outp) => {
            return if outp.status.success() {
                String::from_utf8(outp.stdout).unwrap_or(no_branch.to_string()).trim().to_string()
            } else {
                no_branch.to_string()
            };
        },
        Err(_) => no_branch,
    };

    result.to_string()
}

fn execute_command_w_output(args: Vec<&str>) -> Result<Output, Error> {
    let child = Command::new("git")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.wait_with_output()
}
