use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::{self, Command};

extern crate fs_extra;
extern crate hubcaps;
extern crate skeleton_parser;
extern crate test_runner;
extern crate tokio;

use term_painter::Color::*;
use term_painter::ToStyle;

use std::fs::OpenOptions;
use std::io::Write;

use hubcaps::{Credentials, Github, Result};
use std::env;
use tokio::runtime::Runtime;

use hubcaps::repositories::{RepoOptions, RepoOptionsBuilder, Repositories};
use skeleton_parser::{return_default_delim, SkeletonCode, SkeletonDelimiters};
use test_runner::broker_test;

//returns a command setup ready to run the tests
fn command_wrapper(test_command: &str, command_directory: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    command.current_dir(command_directory);
    command
}
//Copy-Item C:\Logfiles -Destination C:\Drawings\Logs -Recurse
pub fn duplicate_directory(src: &str, dest: &str) {
    if cfg!(target_os = "windows") {
        panic!("need to support windows");
    }

    let owned_string: String = "cp -r ".to_owned();
    let command = owned_string + src + " " + dest;
    let mut c = command_wrapper(&command, ".");
    c.output();
}

fn write_file(filepath: &str, contents: &str) {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filepath)
    {
        Ok(ref mut file) => {
            file.set_len(0);
            writeln!(file, "{}",contents).unwrap();
        }
        Err(err) => {
            panic!("Failed to open log file: {}", err);
        }
    }
}

pub fn replace_with_skeleton(filepath: &str) {
    let contents = match fs::read_to_string(&filepath) {
        Ok(contents) => contents,
        Err(_) => return,
    };
    let delims = return_default_delim();
    let parsed_code = SkeletonCode::new(delims, contents).unwrap();
    write_file(filepath, &parsed_code.skeleton_code);
}

pub fn replace_with_solution(filepath: &str) {
    let contents = match fs::read_to_string(&filepath) {
        Ok(contents) => contents,
        Err(_) => return,
    };

    let delims = SkeletonDelimiters {
        skeleton_tag: "!_SKELETON".to_string(),
        skeleton_delimiter: "#//!_ ".to_string(),
        solution_tag: "!_SOLUTION".to_string(),
    };
    let parsed_code = SkeletonCode::new(delims, contents).unwrap();
    write_file(filepath, &parsed_code.solution_code);
}

pub struct GithubCommand {
    token: String,
}

impl GithubCommand {
    pub fn new(token: &str) -> GithubCommand {
        let gh = GithubCommand {
            token: token.to_string(),
        };
        gh
    }
    pub fn create_repo(&self, name: &str, description: &str) {
        let github = Github::new(
            "myreplicatedu/0.0.1",
            Credentials::Token(self.token.clone()),
        );
        let ro = RepoOptions {
            name: name.to_string(),
            description: Some(description.to_string()),
            homepage: Some("N/A".to_string()),
            private: Some(false),
            has_issues: Some(true),
            has_wiki: Some(true),
            has_downloads: Some(true),
            team_id: Some(0),
            auto_init: Some(false),
            gitignore_template: Some("".to_string()),
            license_template: Some("BSD".to_string()),
        };

        github.repos().create(&ro);
    }
}

pub fn pull_class_repo(repopath: &str, folder: &str) {
    let owned_string: String = "git clone ".to_owned();
    let command = owned_string + repopath;
    let mut c = command_wrapper(&command, folder);
    c.output();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn echo_hello() {
        let mut c = command_wrapper("ls", "..");
        dbg!(c.output());
    }
}
